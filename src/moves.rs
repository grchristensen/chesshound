#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// A simple way to represent chess moves by a string containing the moves in algebraic notation.
pub struct AlgebraicMove(String);

/// An interface for moves that can be converted from algebraic chess notation.
pub trait Move {
    /// Returns `Ok(move)` if the given algebraic notation is valid, and `Err(san_error)` if it
    /// isn't.
    fn try_from_algebraic(algebraic: String) -> Result<Self, SANError>
    where
        Self: Sized;

    /// Returns a new move from the given algebraic notation.
    ///
    /// # Panics
    ///
    /// Panics if `algebraic` is not valid algebraic notation.
    fn from_algebraic(algebraic: String) -> Self
    where
        Self: Sized,
    {
        Self::try_from_algebraic(algebraic).unwrap()
    }

    /// Returns the algebraic notation represented by this move.
    fn to_algebraic(self) -> String;
}

/// Errors related to invalid algebraic notation.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SANError {
    message: String,
}

impl SANError {
    pub fn new(message: String) -> SANError {
        SANError { message: message }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl AlgebraicMove {
    fn is_effect(symbol: char) -> bool {
        symbol == '+' || symbol == '#'
    }

    fn is_coordinate(coordinate: &str) -> Result<(), SANError> {
        let file = coordinate.chars().nth(0).unwrap();
        let rank = coordinate.chars().nth(1).unwrap();

        AlgebraicMove::is_file(file)?;
        AlgebraicMove::is_rank(rank)?;

        Ok(())
    }

    fn is_file(file: char) -> Result<(), SANError> {
        if file < 'a' || file > 'h' {
            Err(SANError::new(String::from(format!(
                "Invalid file: {file}",
                file = file
            ))))
        } else {
            Ok(())
        }
    }

    fn is_rank(rank: char) -> Result<(), SANError> {
        if rank < '1' || rank > '8' {
            Err(SANError::new(String::from(format!(
                "Invalid rank: {rank}",
                rank = rank
            ))))
        } else {
            Ok(())
        }
    }

    fn is_rank_or_file(rank: char) -> Result<(), SANError> {
        let is_rank = AlgebraicMove::is_rank(rank).is_ok();
        let is_file = AlgebraicMove::is_file(rank).is_ok();

        if !is_rank && !is_file {
            Err(SANError::new(String::from(format!(
                "Invalid rank/file: {rank}",
                rank = rank
            ))))
        } else {
            Ok(())
        }
    }

    fn is_piece(piece: char) -> Result<(), SANError> {
        if !"NBRQK".contains(piece) {
            Err(SANError::new(String::from(format!(
                "Invalid piece: {piece}",
                piece = piece
            ))))
        } else {
            Ok(())
        }
    }

    fn is_piece_or_file(piece: char) -> Result<(), SANError> {
        let is_piece = AlgebraicMove::is_piece(piece).is_ok();
        let is_file = AlgebraicMove::is_file(piece).is_ok();

        if !is_piece && !is_file {
            Err(SANError::new(String::from(format!(
                "Invalid piece/file: {piece}",
                piece = piece
            ))))
        } else {
            Ok(())
        }
    }

    fn is_takes(takes: char) -> Result<(), SANError> {
        if takes != 'x' {
            Err(SANError::new(String::from("Invalid takes symbol: x")))
        } else {
            Ok(())
        }
    }

    fn is_specified_piece(piece: char) -> Result<(), SANError> {
        AlgebraicMove::is_piece(piece)?;

        if piece == 'K' {
            // This would imply two or more kings, which is not allowed.
            Err(SANError::new(String::from("Invalid piece: K")))
        } else {
            Ok(())
        }
    }

    fn is_promotion(symbol: char) -> Result<(), SANError> {
        if symbol != '=' {
            Err(SANError::new(String::from(format!(
                "Invalid promotion symbol: {symbol}",
                symbol = symbol
            ))))
        } else {
            Ok(())
        }
    }
}

impl Move for AlgebraicMove {
    fn try_from_algebraic(algebraic: String) -> Result<AlgebraicMove, SANError> {
        let mut test_algebraic = algebraic.clone();

        if let Some(last_char) = test_algebraic.chars().last() {
            if AlgebraicMove::is_effect(last_char) {
                test_algebraic.pop();
            }

            let move_length = test_algebraic.len();

            if move_length == 0 || move_length == 1 {
                return Err(SANError::new(String::from(format!(
                    "Move is too short: {algebraic}",
                    algebraic = algebraic
                ))));
            } else if move_length == 2 {
                // Should be a typical pawn move, so only a coordinate is specified.
                AlgebraicMove::is_coordinate(&test_algebraic)?;
            } else if move_length == 3 {
                // Unless its short castles, it should be a typical piece move, consisting of a
                // piece and coordinate.
                if &test_algebraic != "O-O" {
                    AlgebraicMove::is_piece(test_algebraic.remove(0))?;
                    AlgebraicMove::is_coordinate(&test_algebraic)?;
                }
            } else if move_length == 4 {
                // Three possible types of moves.
                if test_algebraic.chars().nth(1).unwrap() == 'x' {
                    // Typical capture.
                    test_algebraic.remove(1);
                    AlgebraicMove::is_piece_or_file(test_algebraic.remove(0))?;
                    AlgebraicMove::is_coordinate(&test_algebraic)?;
                } else if test_algebraic.chars().nth(2).unwrap() == '=' {
                    // Typical promotion move.
                    AlgebraicMove::is_piece(test_algebraic.remove(3))?;
                    AlgebraicMove::is_promotion(test_algebraic.remove(2))?;
                    AlgebraicMove::is_coordinate(&test_algebraic)?;
                } else {
                    // Move where two pieces can reach same square and a file/rank is specified.
                    AlgebraicMove::is_rank_or_file(test_algebraic.remove(1))?;
                    AlgebraicMove::is_specified_piece(test_algebraic.remove(0))?;
                    AlgebraicMove::is_coordinate(&test_algebraic)?;
                }
            } else if move_length == 5 {
                // Either long castles or a specifying capture move.
                if &test_algebraic != "O-O-O" {
                    AlgebraicMove::is_takes(test_algebraic.remove(2))?;
                    AlgebraicMove::is_rank_or_file(test_algebraic.remove(1))?;
                    AlgebraicMove::is_specified_piece(test_algebraic.remove(0))?;
                    AlgebraicMove::is_coordinate(&test_algebraic)?;
                }
            } else if move_length == 6 {
                // Only pawn captures that promote are possible.
                AlgebraicMove::is_piece(test_algebraic.remove(5))?;
                AlgebraicMove::is_promotion(test_algebraic.remove(4))?;
                AlgebraicMove::is_takes(test_algebraic.remove(1))?;
                AlgebraicMove::is_file(test_algebraic.remove(0))?;
                AlgebraicMove::is_coordinate(&test_algebraic)?;
            } else {
                return Err(SANError::new(String::from(format!(
                    "Move is too long: {test_algebraic}",
                    test_algebraic = test_algebraic
                ))));
            }
        } else {
            return Err(SANError::new(String::from("Empty string")));
        }

        Ok(AlgebraicMove(String::from(algebraic)))
    }

    fn to_algebraic(self) -> String {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::{AlgebraicMove, Move};

    #[rstest(
        san_move,
        case("e4"),
        case("Be3"),
        case("Ndf6"),
        case("Kxe2"),
        case("bxc4"),
        case("Raxd8"),
        case("e8=Q"),
        case("bxc8=R"),
        case("O-O"),
        case("O-O-O"),
        case("c4+"),
        case("Qe7#"),
        case("O-O-O#"),
        case("axb8=N+")
    )]
    fn algebraic_move_should_accept_valid_san(san_move: &str) {
        let _algebraic_move = AlgebraicMove::from_algebraic(String::from(san_move));
    }

    #[rstest(
        invalid_san_move,
        expected_message,
        case("D4", "Invalid file: D"),
        case("Pd4", "Invalid piece: P"),
        case("Lxd4", "Invalid piece/file: L"),
        case("5", "Move is too short: 5"),
        case("#", "Move is too short: #"),
        case("Raaaxd8+", "Move is too long: Raaaxd8"),
        case("Rd9#", "Invalid rank: 9"),
        case("Kab2", "Invalid piece: K"),
        case("N9b7", "Invalid rank/file: 9"),
        case("d1=Z", "Invalid piece: Z"),
        case("b8=a", "Invalid piece: a"),
        case("d8/", "Invalid piece: d"),
        case("Nxa8=B", "Invalid file: N"),
        case("", "Empty string")
    )]
    fn algebraic_move_should_reject_invalid_san(invalid_san_move: &str, expected_message: &str) {
        let bad_algebraic_move = AlgebraicMove::try_from_algebraic(String::from(invalid_san_move));

        let san_error = bad_algebraic_move.expect_err("Invalid SAN accepted");

        assert_eq!(san_error.message(), expected_message);
    }

    #[rstest(san_move, case("e4"), case("bxc8#"))]
    fn algebraic_move_should_convert_to_valid_san(san_move: &str) {
        let algebraic_move = AlgebraicMove::from_algebraic(String::from(san_move));

        assert_eq!(&algebraic_move.to_algebraic(), san_move);
    }
}
