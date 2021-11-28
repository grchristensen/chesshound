use std::mem;

use pgn_reader::{RawHeader, SanPlus, Skip, Visitor};

use crate::game::GameResult;

/// A visitor designed to work with the `pgn_reader` crate. Extracts relevant game information from
/// pgn input.
pub struct GameParser {
    pgn_game: PGNGame,
}

impl GameParser {
    /// Creates a new `GameParser`.
    pub fn new() -> GameParser {
        GameParser {
            pgn_game: PGNGame::new(),
        }
    }
}

/// The output of GameParser.
pub struct PGNGame {
    moves: Vec<String>,
    result: Option<GameResult>,
    white_player: Option<String>,
    black_player: Option<String>,
}

impl PGNGame {
    fn new() -> PGNGame {
        PGNGame {
            moves: Vec::new(),
            result: None,
            white_player: None,
            black_player: None,
        }
    }

    /// Returns the moves found within the PGN input.
    pub fn moves(&self) -> &Vec<String> {
        &self.moves
    }

    /// Returns the result found within the PGN input.
    pub fn result(&self) -> Option<GameResult> {
        self.result
    }

    /// Returns the name of the player playing white found within the PGN input.
    pub fn white_player(&self) -> Option<&str> {
        match &self.white_player {
            Some(string) => Some(&string),
            None => None,
        }
    }

    /// Returns the name of the player playing black found within the PGN input.
    pub fn black_player(&self) -> Option<&str> {
        match &self.black_player {
            Some(string) => Some(&string),
            None => None,
        }
    }
}

impl Visitor for GameParser {
    type Result = PGNGame;

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        if key == b"Result" {
            self.pgn_game.result =
                Some(GameResult::from(String::from(value.decode_utf8().unwrap())));
        } else if key == b"White" {
            self.pgn_game.white_player = Some(String::from(value.decode_utf8().unwrap()));
        } else if key == b"Black" {
            self.pgn_game.black_player = Some(String::from(value.decode_utf8().unwrap()));
        }
    }

    fn san(&mut self, san_plus: SanPlus) {
        self.pgn_game.moves.push(san_plus.to_string());
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true)
    }

    fn end_game(&mut self) -> Self::Result {
        mem::replace(&mut self.pgn_game, PGNGame::new())
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::GameParser;

    use pgn_reader::BufferedReader;

    use crate::game::GameResult;

    #[rstest(pgn, expected_moves,
        case(
            b"1. e4 e5 2. Nf3 Nc6",
            vec![
                String::from("e4"),
                String::from("e5"),
                String::from("Nf3"),
                String::from("Nc6"),
            ]
        ),
        case(
            b"1. d4 Nf6 2. c4 g6 3. Nc3",
            vec![
                String::from("d4"),
                String::from("Nf6"),
                String::from("c4"),
                String::from("g6"),
                String::from("Nc3"),
            ]
        )
    )]
    fn game_visitor_should_find_correct_moves(pgn: &[u8], expected_moves: Vec<String>) {
        let mut reader = BufferedReader::new_cursor(&pgn[..]);

        let mut game_parser = GameParser::new();
        let pgn_game = reader.read_game(&mut game_parser).unwrap().unwrap();

        assert_eq!(pgn_game.moves(), &expected_moves);
    }

    #[rstest(
        pgn,
        expected_result,
        case(b"[Result \"0-1\"]\n1. e4 e5 2. Ke2", Some(GameResult::BlackWon)),
        case(
            b"[Result \"1/2-1/2\"]\n1. e4 e5 2. Nf3 Nf6 3. Nxe5",
            Some(GameResult::Draw)
        ),
        case(b"1. e4 e5 2. Nf3 Nf6 3. Nxe5", None)
    )]
    fn game_visitor_should_find_correct_result(pgn: &[u8], expected_result: Option<GameResult>) {
        let mut reader = BufferedReader::new_cursor(&pgn[..]);

        let mut game_parser = GameParser::new();
        let pgn_game = reader.read_game(&mut game_parser).unwrap().unwrap();

        assert_eq!(pgn_game.result(), expected_result);
    }

    #[rstest(
        pgn,
        expected_white_player,
        expected_black_player,
        case(
            b"[White \"J.J. Jameson\"]\n[Black \"Hikaru Nakamura\"]\n1. e4 e5",
            Some("J.J. Jameson"),
            Some("Hikaru Nakamura")
        ),
        case(
            b"[White \"Nick\"]\n[Black \"Paul\"]\n1. e4 e5",
            Some("Nick"),
            Some("Paul")
        ),
        case(b"1. e4 e5 2. Nf3 Nf6 3. Nxe5", None, None)
    )]
    fn game_visitor_should_find_correct_players(
        pgn: &[u8],
        expected_white_player: Option<&str>,
        expected_black_player: Option<&str>,
    ) {
        let mut reader = BufferedReader::new_cursor(&pgn[..]);

        let mut game_parser = GameParser::new();
        let pgn_game = reader.read_game(&mut game_parser).unwrap().unwrap();

        assert_eq!(pgn_game.white_player(), expected_white_player);
        assert_eq!(pgn_game.black_player(), expected_black_player);
    }
}
