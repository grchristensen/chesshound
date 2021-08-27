#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// A simple way to represent chess moves by a string containing the moves in algebraic notation.
pub struct AlgebraicMove(String);

/// An interface for moves that can be converted from algebraic chess notation.
pub trait Move {
    /// Returns a new move from the given algebraic notation.
    ///
    /// # Panics
    ///
    /// Panics if `algebraic` is not valid algebraic notation.
    fn from_algebraic(algebraic: &str) -> Self;
}

impl Move for AlgebraicMove {
    /// Returns a new `AlgebraicMove` from the given algebraic notation.
    fn from_algebraic(algebraic: &str) -> AlgebraicMove {
        AlgebraicMove(String::from(algebraic))
    }
}

// TODO: Tests to make sure only valid algebraic notation is accepted.
