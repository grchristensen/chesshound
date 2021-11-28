//! # Chesshound
//! Chesshound is a library intending to provide "at a glance" analysis of player games along with
//! statistical tools to scrape and analyze large amounts of game data.
//!
//! ## CLI
//! Chesshound exports all of its primary capabilities through a command-line tool that allows
//! users to analyze their games. One of the basic uses of this tool is to take win rate statistics
//! from a set of games in PGN format.
//!
//! ```bash
//! cat example_games.pgn | chesshound stats
//! ```
//!
//! For comprehensive documentation of the CLI tool, see `chesshound --help`.

/// Types and traits for different representations of chess games.
pub mod game;
/// A structure for organizing games based on their moves.
pub mod move_tree;
/// Type and traits for different representations of chess moves.
pub mod moves;
/// Utilities for parsing games from PGN.
pub mod parsing;
/// Functions for getting statistics from sets of games.
pub mod stats;

pub use game::Game;
pub use move_tree::MoveTree;
pub use move_tree::MoveTreeView;
pub use moves::AlgebraicMove;
pub use moves::Move;
pub use parsing::GameParser;
