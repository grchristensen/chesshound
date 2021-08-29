use std::collections::hash_map;
use std::collections::HashMap;
use std::hash::Hash;
use std::slice;

use crate::game::ListMoves;
use crate::moves::Move;

/// A data structure for storing games by moves played. Useful for creating opening explorers.
///
/// # Examples
///
/// ```
/// use chesshound::Game;
/// use chesshound::AlgebraicMove;
/// use chesshound::Move;
/// use chesshound::MoveTree;
///
/// let game_1 = Game::new(vec![
///     AlgebraicMove::from_algebraic("e4"),
///     AlgebraicMove::from_algebraic("e5"),
/// ]);
///
/// let game_2 = Game::new(vec![
///     AlgebraicMove::from_algebraic("e4"),
///     AlgebraicMove::from_algebraic("c5"),
/// ]);
///
/// let game_3 = Game::new(vec![
///     AlgebraicMove::from_algebraic("e4"),
///     AlgebraicMove::from_algebraic("c5"),
///     AlgebraicMove::from_algebraic("Nf3"),
/// ]);
///
/// let move_tree = MoveTree::new(vec![game_1, game_2, game_3]);
///
/// // Contains game_2 and game_3
/// let sicilian_games = move_tree
///     .with_next(&AlgebraicMove::from_algebraic("e4"))
///     .with_next(&AlgebraicMove::from_algebraic("c4"));
/// ```
pub struct MoveTree<M: Clone + Move + Eq + Hash, G: ListMoves<M>> {
    games: Vec<G>,
    game_tree: HashMap<M, Box<MoveTree<M, G>>>,
}

impl<M: Clone + Move + Eq + Hash, G: ListMoves<M>> MoveTree<M, G> {
    /// Creates a new `MoveTree<M, G>` from the provided `games`.
    pub fn new(games: Vec<G>) -> MoveTree<M, G> {
        let mut empty_games: Vec<G> = Vec::new();
        let mut game_tree: HashMap<M, Box<MoveTree<M, G>>> = HashMap::new();

        for game in games {
            let mut moves_iter = game.list_moves();

            if let Some(first_move) = moves_iter.next() {
                let first_game_tree = game_tree.get_mut(&first_move);

                let mut current_position = if let Some(first_game_tree) = first_game_tree {
                    first_game_tree
                } else {
                    // Have to construct hashmap for the first time.
                    game_tree.insert(
                        first_move.clone(),
                        Box::new(MoveTree {
                            games: Vec::new(),
                            game_tree: HashMap::new(),
                        }),
                    );

                    game_tree.get_mut(&first_move).unwrap()
                };

                for move_ in moves_iter {
                    if let None = current_position.game_tree.get(&move_) {
                        // Have to construct hashmap for the first time.
                        current_position.game_tree.insert(
                            move_.clone(),
                            Box::new(MoveTree {
                                games: Vec::new(),
                                game_tree: HashMap::new(),
                            }),
                        );
                    }

                    current_position = current_position.game_tree.get_mut(&move_).unwrap();
                }

                current_position.games.push(game);
            } else {
                empty_games.push(game);
            }
        }

        MoveTree {
            games: empty_games,
            game_tree: game_tree,
        }
    }

    // TODO: It would be nice if there was a function like this that returned all games containing
    // the *position* after this move, but this would require a much more complex way to compare
    // games.
    /// Returns a subset of the move tree where only games with the next move being `chess_move`
    /// are included.
    pub fn with_next(&self, chess_move: &M) -> MoveTreeView<M, G> {
        if let Some(next_position) = self.game_tree.get(chess_move) {
            MoveTreeView::new(Some(next_position))
        } else {
            MoveTreeView::new(None)
        }
    }

    /// Returns a view of this `MoveTree<M, G>`, which can be iterated over to find all games in
    /// the `MoveTree<M, G>`.
    pub fn view(&self) -> MoveTreeView<M, G> {
        MoveTreeView::new(Some(self))
    }
}

#[derive(Clone)]
/// A view of a subtree within a `MoveTree<M, G>`.
pub struct MoveTreeView<'a, M: Clone + Move + Eq + Hash, G: ListMoves<M>> {
    game_tree: Option<&'a MoveTree<M, G>>,
}

impl<'a, M: Clone + Move + Eq + Hash, G: ListMoves<M>> MoveTreeView<'a, M, G> {
    fn new(game_tree: Option<&'a MoveTree<M, G>>) -> MoveTreeView<'a, M, G> {
        MoveTreeView {
            game_tree: game_tree,
        }
    }

    /// Behaves in the same way as `MoveTree::with_next`.
    pub fn with_next(self, chess_move: &M) -> MoveTreeView<'a, M, G> {
        if let Some(game_tree) = self.game_tree {
            if let Some(next_position) = game_tree.game_tree.get(chess_move) {
                MoveTreeView::new(Some(next_position))
            } else {
                MoveTreeView::new(None)
            }
        } else {
            MoveTreeView::new(None)
        }
    }

    /// Returns an iterator to all games represented by the `MoveTreeView<M, G>`.
    pub fn iter(&self) -> Iter<'a, M, G> {
        Iter::new(&self.game_tree)
    }
}

/// An iterator over games in a `MoveTree<M, G>`.
pub struct Iter<'a, M: Clone + Move + Eq + Hash, G: ListMoves<M>> {
    internal: Option<InternalIter<'a, M, G>>,
}

impl<'a, M: Clone + Move + Eq + Hash, G: ListMoves<M>> Iter<'a, M, G> {
    fn new(game_tree: &Option<&'a MoveTree<M, G>>) -> Iter<'a, M, G> {
        if let Some(game_tree) = game_tree {
            Iter {
                internal: Some(InternalIter::new(game_tree)),
            }
        } else {
            Iter { internal: None }
        }
    }
}

impl<'a, M: Clone + Move + Eq + Hash, G: ListMoves<M>> Iterator for Iter<'a, M, G> {
    type Item = &'a G;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = &mut self.internal {
            iter.next()
        } else {
            None
        }
    }
}

struct InternalIter<'a, M: Clone + Move + Eq + Hash, G: ListMoves<M>> {
    vec_iter: slice::Iter<'a, G>,
    node_stack: Vec<hash_map::Iter<'a, M, Box<MoveTree<M, G>>>>,
}

impl<'a, M: Clone + Move + Eq + Hash, G: ListMoves<M>> InternalIter<'a, M, G> {
    fn new(game_tree: &'a MoveTree<M, G>) -> InternalIter<'a, M, G> {
        let node_stack = vec![game_tree.game_tree.iter()];

        InternalIter {
            vec_iter: game_tree.games.iter(),
            node_stack: node_stack,
        }
    }
}

impl<'a, M: Clone + Move + Eq + Hash, G: ListMoves<M>> Iterator for InternalIter<'a, M, G> {
    type Item = &'a G;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(game) = self.vec_iter.next() {
            Some(game)
        } else {
            if let Some(game_tree_iter) = self.node_stack.last_mut() {
                if let Some((_, next_tree)) = game_tree_iter.next() {
                    self.node_stack.push(next_tree.game_tree.iter());
                    self.vec_iter = next_tree.games.iter();
                } else {
                    self.node_stack.pop();
                };

                self.next()
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::test_utils::*;

    use crate::AlgebraicMove;
    use crate::Game;

    fn contains_same_games(
        move_tree_view: MoveTreeView<AlgebraicMove, Game<AlgebraicMove>>,
        mut games: Vec<Game<AlgebraicMove>>,
    ) -> bool {
        for game in move_tree_view.iter() {
            let mut target_index = games.len();
            let mut one_found = false;

            for (index, other_game) in games.iter().enumerate() {
                if game == other_game {
                    target_index = index;
                    one_found = true;
                }
            }

            if !one_found {
                return false;
            }

            if target_index != games.len() {
                games.remove(target_index);
            }
        }

        games.len() == 0
    }

    #[test]
    fn move_trees_should_allow_empty_games() {
        let games = vec![unplayed_game(), unplayed_game(), ruy_lopez()];

        let move_tree = MoveTree::new(games.clone());

        assert!(contains_same_games(
            move_tree.view(),
            vec![unplayed_game(), ruy_lopez(), unplayed_game()]
        ))
    }

    #[test]
    fn move_trees_should_return_subsets() {
        let games = vec![
            italian_game(),
            ruy_lopez(),
            sicilian_naijdorf(),
            sicilian_dragon(),
            queens_gambit(),
        ];
        let move_tree = MoveTree::new(games.clone());

        let sicilian_games = move_tree
            .with_next(&AlgebraicMove::from_algebraic("e4"))
            .with_next(&AlgebraicMove::from_algebraic("c5"));

        assert!(contains_same_games(
            sicilian_games,
            vec![sicilian_naijdorf(), sicilian_dragon()]
        ));

        let d4_games = move_tree.with_next(&AlgebraicMove::from_algebraic("d4"));

        assert!(contains_same_games(d4_games, vec![queens_gambit()]));

        let reversed_sicilian_games = move_tree
            .with_next(&AlgebraicMove::from_algebraic("c4"))
            .with_next(&AlgebraicMove::from_algebraic("e5"));

        assert!(contains_same_games(reversed_sicilian_games, vec![]));
    }
}
