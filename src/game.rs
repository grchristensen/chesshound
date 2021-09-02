use crate::moves::Move;
use crate::parsing::PGNGame;

#[derive(Debug, Clone, PartialEq, Eq)]
/// A generic representation of a chess game.
pub struct Game<M: Move> {
    result: GameResult,
    moves: GameMoves<M>,
    white_player: String,
    black_player: String,
}

impl<M: Move> From<PGNGame> for Game<M> {
    fn from(pgn_game: PGNGame) -> Game<M> {
        let mut moves: Vec<M> = Vec::new();

        for san_move in pgn_game.moves().clone() {
            moves.push(M::from_algebraic(san_move));
        }

        Game {
            result: pgn_game.result().expect("No result in PGN"),
            moves: GameMoves::new(moves),
            white_player: String::from(pgn_game.white_player().expect("No white player in PGN")),
            black_player: String::from(pgn_game.black_player().expect("No black player in PGN")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A way of representing games based on moves played. Implements ListMoves.
pub struct GameMoves<M: Move> {
    moves: Vec<M>,
}

impl<M: Move> GameMoves<M> {
    /// Constructs a new `GameMoves<M>` with no moves played.
    pub fn new(moves: Vec<M>) -> GameMoves<M> {
        GameMoves { moves: moves }
    }
}

/// Interface for types that give the result of a chess game.
pub trait GiveResult {
    /// Returns the result of this game.
    fn result(&self) -> GameResult;
}

/// Enum representing the possible results in a game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    WhiteWon,
    BlackWon,
    Draw,
}

impl<M: Move> GiveResult for Game<M> {
    fn result(&self) -> GameResult {
        self.result.result()
    }
}

impl<M: 'static + Clone + Move> ListMoves<M> for Game<M> {
    fn list_moves(&self) -> Box<dyn Iterator<Item = M>> {
        self.moves.list_moves()
    }
}

impl GiveResult for GameResult {
    fn result(&self) -> GameResult {
        *self
    }
}

impl From<String> for GameResult {
    fn from(string: String) -> GameResult {
        if &string == "1-0" {
            GameResult::WhiteWon
        } else if &string == "0-1" {
            GameResult::BlackWon
        } else if &string == "1/2-1/2" {
            GameResult::Draw
        } else {
            panic!("Invalid result format: {result}", result = string)
        }
    }
}

/// An interface for listing moves within a game or similar structure.
pub trait ListMoves<M: Clone + Move> {
    /// Returns an iterator of all moves within the type.
    fn list_moves(&self) -> Box<dyn Iterator<Item = M>>;
}

impl<M: 'static + Clone + Move> ListMoves<M> for GameMoves<M> {
    fn list_moves(&self) -> Box<dyn Iterator<Item = M>> {
        Box::new(self.moves.clone().into_iter())
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;
    use crate::AlgebraicMove;
    use test_utils::*;

    type AlgebraicGame = GameMoves<AlgebraicMove>;

    #[rstest(
        game,
        same_game,
        case(queens_gambit(), queens_gambit()),
        case(sicilian_naijdorf(), sicilian_naijdorf())
    )]
    fn game_equality(game: AlgebraicGame, same_game: AlgebraicGame) {
        assert_eq!(game, same_game);
    }

    #[rstest(
        game,
        other_game,
        case(sicilian_naijdorf(), italian_game()),
        case(queens_gambit(), ruy_lopez())
    )]
    fn game_inequality(game: AlgebraicGame, other_game: AlgebraicGame) {
        assert_ne!(game, other_game);
    }
}

#[cfg(test)]
pub mod test_utils {
    use crate::game::GameMoves;
    use crate::moves::Move;
    use crate::AlgebraicMove;

    pub mod results {
        use super::super::GameResult;

        pub fn white_won() -> GameResult {
            GameResult::WhiteWon
        }

        pub fn black_won() -> GameResult {
            GameResult::BlackWon
        }

        pub fn draw() -> GameResult {
            GameResult::Draw
        }
    }

    pub fn unplayed_game() -> GameMoves<AlgebraicMove> {
        GameMoves::new(Vec::new())
    }

    pub fn italian_game() -> GameMoves<AlgebraicMove> {
        GameMoves::new(vec![
            AlgebraicMove::from_algebraic(String::from("e4")),
            AlgebraicMove::from_algebraic(String::from("e5")),
            AlgebraicMove::from_algebraic(String::from("Nf3")),
            AlgebraicMove::from_algebraic(String::from("Nc6")),
            AlgebraicMove::from_algebraic(String::from("Bc4")),
        ])
    }

    pub fn ruy_lopez() -> GameMoves<AlgebraicMove> {
        GameMoves::new(vec![
            AlgebraicMove::from_algebraic(String::from("e4")),
            AlgebraicMove::from_algebraic(String::from("e5")),
            AlgebraicMove::from_algebraic(String::from("Nf3")),
            AlgebraicMove::from_algebraic(String::from("Nc6")),
            AlgebraicMove::from_algebraic(String::from("Bb5")),
        ])
    }

    pub fn sicilian_naijdorf() -> GameMoves<AlgebraicMove> {
        GameMoves::new(vec![
            AlgebraicMove::from_algebraic(String::from("e4")),
            AlgebraicMove::from_algebraic(String::from("c5")),
            AlgebraicMove::from_algebraic(String::from("Nf3")),
            AlgebraicMove::from_algebraic(String::from("d3")),
            AlgebraicMove::from_algebraic(String::from("e5")),
            AlgebraicMove::from_algebraic(String::from("cxe5")),
            AlgebraicMove::from_algebraic(String::from("Nxe5")),
            AlgebraicMove::from_algebraic(String::from("Nf6")),
            AlgebraicMove::from_algebraic(String::from("Nc3")),
            AlgebraicMove::from_algebraic(String::from("a6")),
        ])
    }

    pub fn sicilian_dragon() -> GameMoves<AlgebraicMove> {
        GameMoves::new(vec![
            AlgebraicMove::from_algebraic(String::from("e4")),
            AlgebraicMove::from_algebraic(String::from("c5")),
            AlgebraicMove::from_algebraic(String::from("Nf3")),
            AlgebraicMove::from_algebraic(String::from("d3")),
            AlgebraicMove::from_algebraic(String::from("e5")),
            AlgebraicMove::from_algebraic(String::from("cxe5")),
            AlgebraicMove::from_algebraic(String::from("Nxe5")),
            AlgebraicMove::from_algebraic(String::from("Nf6")),
            AlgebraicMove::from_algebraic(String::from("Nc3")),
            AlgebraicMove::from_algebraic(String::from("g6")),
        ])
    }

    pub fn queens_gambit() -> GameMoves<AlgebraicMove> {
        GameMoves::new(vec![
            AlgebraicMove::from_algebraic(String::from("d4")),
            AlgebraicMove::from_algebraic(String::from("d5")),
            AlgebraicMove::from_algebraic(String::from("c4")),
        ])
    }
}
