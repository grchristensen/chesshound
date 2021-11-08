use async_trait::async_trait;
use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct MonthlyArchive {
    games: Vec<Game>,
}

#[derive(Deserialize)]
struct Game {
    pgn: String,
    end_time: i64,
}

/// Represents the different errors that can happen when making requests to an API.
#[derive(Debug)]
pub enum APIError {
    ClientError(u16, String),
    Connection(String),
    Decode,
    Timeout,
    Unknown(String),
}

/// Interface for downloading games from an API.
#[async_trait]
pub trait GetGames {
    /// Returns the PGN for all games from the given player for the given time period.
    async fn get_games(
        &self,
        username: &str,
        from: DateTime<Utc>,
        until: DateTime<Utc>,
    ) -> Result<String, APIError>;
}

/// Allows for interaction with the chess.com public data API. Implements `GetGames`.
pub struct ChessComAPI {
    root: String,
    client: Client,
}

impl ChessComAPI {
    /// Constructs an instance of the chess.com API assuming the given API root. `root` should not
    /// have a trailing forward slash.
    pub fn new(root: String) -> ChessComAPI {
        ChessComAPI {
            root: root,
            client: Client::new(),
        }
    }

    /// Returns the monthly archive for the given username and time. Any time within the desired
    /// month can be used.
    async fn monthly_archive(
        &self,
        username: &str,
        time: &DateTime<Utc>,
    ) -> Result<String, APIError> {
        Ok(concat_pgns(
            self.request_monthly_archive(username, time.year(), time.month())
                .await?,
        ))
    }

    /// Returns all games in PGN for that occur within the month of `begin_time` and only after
    /// `begin_time`.
    async fn monthly_archive_after(
        &self,
        username: &str,
        begin_time: &DateTime<Utc>,
    ) -> Result<String, APIError> {
        let games = self.request_monthly_archive(username, begin_time.year(), begin_time.month());

        let game_pgns: Vec<String> = games
            .await?
            .into_iter()
            .filter_map(|game| {
                // We want all games that ended at or after begin_time.
                if Utc.timestamp(game.end_time, 0) >= *begin_time {
                    Some(game.pgn)
                } else {
                    None
                }
            })
            .collect();

        Ok(game_pgns.join("\n\n"))
    }

    /// Returns all games in PGN for that occur within the month of `end_time` and only before
    /// `end_time`.
    async fn monthly_archive_before(
        &self,
        username: &str,
        end_time: &DateTime<Utc>,
    ) -> Result<String, APIError> {
        let games = self.request_monthly_archive(username, end_time.year(), end_time.month());

        let game_pgns: Vec<String> = games
            .await?
            .into_iter()
            .filter_map(|game| {
                // We want all games that ended before but not at end_time.
                if Utc.timestamp(game.end_time, 0) < *end_time {
                    Some(game.pgn)
                } else {
                    None
                }
            })
            .collect();

        Ok(game_pgns.join("\n\n"))
    }

    /// Makes the request to chess.com for the player's monthly archive of games.
    async fn request_monthly_archive(
        &self,
        username: &str,
        year: i32,
        month: u32,
    ) -> Result<Vec<Game>, APIError> {
        // See https://www.chess.com/news/view/published-data-api#pubapi-endpoint-games-archive
        let response = self
            .client
            .get(format!(
                "{}/pub/player/{}/games/{:04}/{:02}",
                self.root, username, year, month
            ))
            .send()
            .await;

        let response = match response {
            Ok(response) => response,
            Err(e) => {
                // Currently only anticipating errors from timeout or connections to a bad root.
                if e.is_timeout() {
                    return Err(APIError::Timeout);
                } else if e.is_connect() {
                    return Err(APIError::Connection(
                        e.url()
                            .expect("Got connection error with no url")
                            .to_string(),
                    ));
                } else {
                    return Err(APIError::Unknown(format!("{}", e)));
                }
            }
        };

        let status = response.status();

        // Not yet anticipating a server error.
        if status.is_client_error() {
            return Err(APIError::ClientError(
                status.as_u16(),
                status
                    .canonical_reason()
                    .expect("Got client error with no reason phrase")
                    .to_string(),
            ));
        }

        let MonthlyArchive { games } = match response.json::<MonthlyArchive>().await {
            Ok(monthly_archive) => monthly_archive,
            Err(e) => {
                // Only expecting errors from decoding into json.
                if e.is_decode() {
                    return Err(APIError::Decode);
                } else {
                    return Err(APIError::Unknown(format!("{}", e)));
                }
            }
        };

        Ok(games)
    }
}

#[async_trait]
impl GetGames for ChessComAPI {
    async fn get_games(
        &self,
        username: &str,
        from: DateTime<Utc>,
        until: DateTime<Utc>,
    ) -> Result<String, APIError> {
        // Primary strategy of this function is to make a request for each month that falls within
        // the given time range and then filter out any extra games that don't fit the range.
        let mut month_pgns: Vec<String> = Vec::new();

        month_pgns.push(self.monthly_archive_after(username, &from).await?);

        let mut current_month = add_month(&time_truncate(&from));
        let truncated_end_time = time_truncate(&until);

        // In addition to the first and last month, we need to get all the months in between.
        while current_month < truncated_end_time {
            month_pgns.push(self.monthly_archive(username, &current_month).await?);
            current_month = add_month(&current_month);
        }

        if truncated_end_time < until {
            month_pgns.push(self.monthly_archive_before(username, &until).await?);
        }

        Ok(month_pgns.join("\n\n"))
    }
}

fn concat_pgns(games: Vec<Game>) -> String {
    let game_pgns: Vec<String> = games.into_iter().map(|game| game.pgn).collect();

    game_pgns.join("\n\n")
}

/// `time` should be the earliest time within that month.
fn add_month(time: &DateTime<Utc>) -> DateTime<Utc> {
    // Adding 5 weeks ensures that we get within the next month no matter how many days are in the
    // current month.
    time_truncate(&(*time + Duration::weeks(5)))
}

/// Returns the earliest time within the month by keeping the year and month but setting the day to
/// be the first and the time to be 00:00:00.
fn time_truncate(time: &DateTime<Utc>) -> DateTime<Utc> {
    Utc.ymd(time.year(), time.month(), 1).and_hms(0, 0, 0)
}
