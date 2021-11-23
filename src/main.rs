use io::Read;
use std::io;

use chrono::{DateTime, Utc};
use clap::{App, Arg, SubCommand};
use pgn_reader::BufferedReader;

use chesshound::moves::SANError;
use chesshound::scraping::{APIError, ChessComAPI, GetGames};
use chesshound::{stats, AlgebraicMove, Game, GameParser, Move, MoveTree};

#[tokio::main]
async fn main() -> io::Result<()> {
    let matches = App::new("Chesshound")
        .version("0.1.0")
        .author("Gage C. <github.com/grchristensen>")
        .about("Chesshound is a CLI tool for finding patterns in sets of chess games")
        .subcommand(
            SubCommand::with_name("stats")
                .about("Takes PGN from standard input and gives statistics on games found")
                .arg(
                    Arg::with_name("branches")
                        .help("Show all moves that occur after this one in the game set")
                        .short("b")
                        .long("branches"),
                )
                .arg(
                    Arg::with_name("MOVES")
                        .help("Filters games by moves played")
                        .index(1)
                        .multiple(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("scrape")
                .about("Collects games from chess.com and prints them to standard output")
                .arg(
                    Arg::with_name("PLAYER")
                        .help("The player account to collect games from")
                        .index(1),
                )
                .arg(
                    Arg::with_name("BEGIN_TIME")
                        .help("All games collected will end at or after this time")
                        .index(2),
                )
                .arg(
                    Arg::with_name("END_TIME")
                        .help("All games collected will end before or at this time")
                        .index(3),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("stats", Some(matches)) => {
            let pgn = io::stdin()
                .lock()
                .bytes()
                .map(|b| b.unwrap())
                .collect::<Vec<_>>();

            let moves: Vec<String> = if let Some(values) = matches.values_of("MOVES") {
                values.map(|move_| String::from(move_)).collect::<Vec<_>>()
            } else {
                Vec::new()
            };

            let show_branches = matches.is_present("branches");

            let stats = match run_stats(&pgn, moves, show_branches) {
                Ok(stats) => stats,
                Err(e) => {
                    // TODO: Refactor so that this error can specifically point out which move was
                    // incorrect.
                    eprintln!("{}", e.message());
                    return Ok(());
                }
            };

            println!("{}", stats);
        }
        ("scrape", Some(matches)) => {
            let player = matches.value_of("PLAYER").unwrap();
            let begin_time = matches.value_of("BEGIN_TIME").unwrap();
            let end_time = matches.value_of("END_TIME").unwrap();

            let begin_time = match begin_time.parse::<DateTime<Utc>>() {
                Ok(begin_time) => begin_time,
                Err(_) => {
                    eprintln!("Could not parse beginning time as ISO 8601");
                    return Ok(());
                }
            };
            let end_time = match end_time.parse::<DateTime<Utc>>() {
                Ok(end_time) => end_time,
                Err(_) => {
                    eprintln!("Could not parse end time as ISO 8601");
                    return Ok(());
                }
            };

            let api = ChessComAPI::new(String::from("https://api.chess.com"));

            let pgn = match api.get_games(player, begin_time, end_time).await {
                Ok(pgn) => pgn,
                Err(e) => {
                    match e {
                        APIError::ClientError(code, reason) => {
                            eprintln!("Client Error: {} {}", code, reason);
                        }
                        APIError::Connection(url) => {
                            eprintln!("Could not connect to {}", url);
                        }
                        APIError::Decode => {
                            eprintln!("Could not decode API response as JSON");
                        }
                        APIError::Timeout => {
                            eprintln!("Request for monthly archive timed out");
                        }
                        APIError::Unknown(message) => {
                            panic!("Unexpected error: {}", message);
                        }
                    };

                    return Ok(());
                }
            };

            println!("{}", pgn);
        }
        _ => {}
    };

    Ok(())
}

fn run_stats(pgn: &[u8], moves: Vec<String>, show_branches: bool) -> Result<String, SANError> {
    let mut reader = BufferedReader::new_cursor(&pgn[..]);

    fn read_game<R: Read>(reader: &mut BufferedReader<R>) -> Option<Game<AlgebraicMove>> {
        let mut game_parser = GameParser::new();
        let pgn_game = reader.read_game(&mut game_parser).unwrap();

        pgn_game.map(|g| Game::<AlgebraicMove>::from(g))
    }

    let mut games: Vec<Game<AlgebraicMove>> = Vec::new();

    while let Some(game) = read_game(&mut reader) {
        games.push(game);
    }

    let move_tree = MoveTree::new(games);
    let mut move_tree_view = move_tree.view();

    for move_ in moves {
        let move_ = AlgebraicMove::try_from_algebraic(move_)?;
        move_tree_view = move_tree_view.with_next(&move_);
    }

    let branches: Vec<String> = if show_branches {
        let mut branches = match move_tree_view.branches() {
            Some(branches) => branches.map(|move_| move_.clone().to_algebraic()).collect(),
            None => Vec::new(),
        };

        branches.sort();

        branches
    } else {
        Vec::new()
    };

    let filtered_games = move_tree_view.iter().collect::<Vec<_>>();
    let game_count = filtered_games.len();

    let (white_win_rate, black_win_rate, draw_rate) =
        stats::results(&mut filtered_games.into_iter());
    let white_win_percent = white_win_rate * 100.;
    let black_win_percent = black_win_rate * 100.;
    let draw_percent = draw_rate * 100.;

    let mut output: String = format!(
        "{} games\nWhite Wins: {:.2}%\nBlack Wins: {:.2}%\nDraw: {:.2}%",
        game_count, white_win_percent, black_win_percent, draw_percent
    )
    .to_owned();

    if show_branches {
        let branches_output = if branches.len() > 0 {
            let mut branches_output = "Moves:".to_owned();

            for branch in branches {
                branches_output += &(" ".to_owned() + &branch);
            }

            branches_output
        } else {
            "No moves".to_owned()
        };

        output = output + "\n" + &branches_output;
    }

    Ok(output)
}
