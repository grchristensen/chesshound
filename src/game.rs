use crate::moves::Move;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
/// A way of representing games based on moves played. Implements ListMoves.
pub struct Game<M: Move> {
    moves: Vec<M>,
}

impl<M: Move> Game<M> {
    /// Constructs a new `Game<M>` with no moves played.
    pub fn new(moves: Vec<M>) -> Game<M> {
        Game { moves: moves }
    }
}

/// Interface for types that give the result of a chess game.
pub trait GiveResult {
    /// Returns the result of this game.
    fn result(&self) -> GameResult;
}

/// Enum representing the possible results in a game.
#[derive(Clone, Copy)]
pub enum GameResult {
    WhiteWon,
    BlackWon,
    Draw,
    Aborted,
}

impl GiveResult for GameResult {
    fn result(&self) -> GameResult {
        *self
    }
}

/// An interface for listing moves within a game or similar structure.
pub trait ListMoves<M: Clone + Move> {
    /// Returns an iterator of all moves within the type.
    fn list_moves(&self) -> Box<dyn Iterator<Item = M>>;
}

impl<M: 'static + Clone + Move> ListMoves<M> for Game<M> {
    /// Returns an iterator of all moves within the `Game<M>`.
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

    type AlgebraicGame = Game<AlgebraicMove>;

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
    use crate::moves::Move;
    use crate::AlgebraicMove;
    use crate::Game;

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

        pub fn aborted() -> GameResult {
            GameResult::Aborted
        }
    }

    pub fn unplayed_game() -> Game<AlgebraicMove> {
        Game::new(Vec::new())
    }

    pub fn italian_game() -> Game<AlgebraicMove> {
        Game::new(vec![
            AlgebraicMove::from_algebraic(String::from("e4")),
            AlgebraicMove::from_algebraic(String::from("e5")),
            AlgebraicMove::from_algebraic(String::from("Nf3")),
            AlgebraicMove::from_algebraic(String::from("Nc6")),
            AlgebraicMove::from_algebraic(String::from("Bc4")),
        ])
    }

    pub fn ruy_lopez() -> Game<AlgebraicMove> {
        Game::new(vec![
            AlgebraicMove::from_algebraic(String::from("e4")),
            AlgebraicMove::from_algebraic(String::from("e5")),
            AlgebraicMove::from_algebraic(String::from("Nf3")),
            AlgebraicMove::from_algebraic(String::from("Nc6")),
            AlgebraicMove::from_algebraic(String::from("Bb5")),
        ])
    }

    pub fn sicilian_naijdorf() -> Game<AlgebraicMove> {
        Game::new(vec![
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

    pub fn sicilian_dragon() -> Game<AlgebraicMove> {
        Game::new(vec![
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

    pub fn queens_gambit() -> Game<AlgebraicMove> {
        Game::new(vec![
            AlgebraicMove::from_algebraic(String::from("d4")),
            AlgebraicMove::from_algebraic(String::from("d5")),
            AlgebraicMove::from_algebraic(String::from("c4")),
        ])
    }
}
