//! # Chesshound
//! Chesshound is a library intending to provide "at a glance" analysis of player games along with
//! statistical tools to scrape and analyze large amounts of game data.

/// Types and traits for different representations of chess games.
pub mod game;
/// A structure for organizing games based on their moves.
pub mod move_tree;
/// Type and traits for different representations of chess moves.
pub mod moves;

pub use game::Game;
pub use move_tree::MoveTree;
pub use move_tree::MoveTreeView;
pub use moves::AlgebraicMove;
pub use moves::Move;
