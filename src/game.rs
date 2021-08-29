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
    use super::*;
    use test_utils::*;

    #[test]
    fn game_equality() {
        assert_eq!(queens_gambit(), queens_gambit());
        assert_eq!(sicilian_naijdorf(), sicilian_naijdorf());
        assert_ne!(sicilian_naijdorf(), italian_game());
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
            AlgebraicMove::from_algebraic("e4"),
            AlgebraicMove::from_algebraic("e5"),
            AlgebraicMove::from_algebraic("Nf3"),
            AlgebraicMove::from_algebraic("Nc6"),
            AlgebraicMove::from_algebraic("Bc4"),
        ])
    }

    pub fn ruy_lopez() -> Game<AlgebraicMove> {
        Game::new(vec![
            AlgebraicMove::from_algebraic("e4"),
            AlgebraicMove::from_algebraic("e5"),
            AlgebraicMove::from_algebraic("Nf3"),
            AlgebraicMove::from_algebraic("Nc6"),
            AlgebraicMove::from_algebraic("Bb5"),
        ])
    }

    pub fn sicilian_naijdorf() -> Game<AlgebraicMove> {
        Game::new(vec![
            AlgebraicMove::from_algebraic("e4"),
            AlgebraicMove::from_algebraic("c5"),
            AlgebraicMove::from_algebraic("Nf3"),
            AlgebraicMove::from_algebraic("d3"),
            AlgebraicMove::from_algebraic("e5"),
            AlgebraicMove::from_algebraic("cxe5"),
            AlgebraicMove::from_algebraic("Nxe5"),
            AlgebraicMove::from_algebraic("Nf6"),
            AlgebraicMove::from_algebraic("Nc3"),
            AlgebraicMove::from_algebraic("a6"),
        ])
    }

    pub fn sicilian_dragon() -> Game<AlgebraicMove> {
        Game::new(vec![
            AlgebraicMove::from_algebraic("e4"),
            AlgebraicMove::from_algebraic("c5"),
            AlgebraicMove::from_algebraic("Nf3"),
            AlgebraicMove::from_algebraic("d3"),
            AlgebraicMove::from_algebraic("e5"),
            AlgebraicMove::from_algebraic("cxe5"),
            AlgebraicMove::from_algebraic("Nxe5"),
            AlgebraicMove::from_algebraic("Nf6"),
            AlgebraicMove::from_algebraic("Nc3"),
            AlgebraicMove::from_algebraic("g6"),
        ])
    }

    pub fn queens_gambit() -> Game<AlgebraicMove> {
        Game::new(vec![
            AlgebraicMove::from_algebraic("d4"),
            AlgebraicMove::from_algebraic("d5"),
            AlgebraicMove::from_algebraic("c4"),
        ])
    }
}
