use crate::game::GameResult;
use crate::game::GiveResult;

/// Returns the percentage of white wins, black wins, draws, and aborts in `game_iter`.
pub fn results<'a, G: GiveResult>(
    game_iter: &mut dyn Iterator<Item = &'a G>,
) -> (f64, f64, f64, f64) {
    let mut white_wins = 0.;
    let mut black_wins = 0.;
    let mut draws = 0.;
    let mut aborts = 0.;

    while let Some(game) = game_iter.next() {
        match game.result() {
            GameResult::WhiteWon => {
                white_wins += 1.;
            }
            GameResult::BlackWon => {
                black_wins += 1.;
            }
            GameResult::Draw => {
                draws += 1.;
            }
            GameResult::Aborted => {
                aborts += 1.;
            }
        };
    }

    let total_games: f64 = white_wins + black_wins + draws + aborts;

    (
        white_wins / total_games,
        black_wins / total_games,
        draws / total_games,
        aborts / total_games,
    )
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use float_cmp::approx_eq;

    use crate::game::test_utils::results::*;
    use crate::game::GameResult;

    use super::results;

    fn close(a: f64, b: f64) -> bool {
        approx_eq!(f64, a, b, epsilon = 0.00000001)
    }

    fn more_white_wins() -> Vec<GameResult> {
        vec![
            white_won(),
            white_won(),
            aborted(),
            black_won(),
            draw(),
            white_won(),
        ]
    }

    fn more_black_wins() -> Vec<GameResult> {
        vec![
            draw(),
            white_won(),
            aborted(),
            black_won(),
            black_won(),
            draw(),
            white_won(),
            black_won(),
        ]
    }

    fn more_draws() -> Vec<GameResult> {
        vec![
            black_won(),
            draw(),
            draw(),
            draw(),
            draw(),
            white_won(),
            black_won(),
        ]
    }

    fn more_aborts() -> Vec<GameResult> {
        vec![
            black_won(),
            aborted(),
            white_won(),
            draw(),
            aborted(),
            aborted(),
        ]
    }

    #[rstest(game_list, expected_results,
        case(more_white_wins(), (0.5, 1. / 6., 1. / 6., 1. / 6.)),
        case(more_black_wins(), (0.25, 0.375, 0.25, 0.125)),
        case(more_draws(), (1. / 7., 2. / 7., 4. / 7., 0.)),
        case(more_aborts(), (1. / 6., 1. / 6., 1. / 6., 0.5)),
    )]
    fn results_should_give_correct_rates(
        game_list: Vec<GameResult>,
        expected_results: (f64, f64, f64, f64),
    ) {
        let (white_win_rate, black_win_rate, draw_rate, abort_rate) =
            results(&mut game_list.iter());

        assert!(close(white_win_rate, expected_results.0));
        assert!(close(black_win_rate, expected_results.1));
        assert!(close(draw_rate, expected_results.2));
        assert!(close(abort_rate, expected_results.3));
    }
}
