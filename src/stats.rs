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
    use float_cmp::approx_eq;

    use crate::game::test_utils::results::*;

    use super::results;

    fn close(a: f64, b: f64) -> bool {
        approx_eq!(f64, a, b, epsilon = 0.00000001)
    }

    #[test]
    fn correct_win_rates() {
        let white_wins = vec![
            white_won(),
            white_won(),
            aborted(),
            black_won(),
            draw(),
            white_won(),
        ];
        let black_wins = vec![
            draw(),
            white_won(),
            aborted(),
            black_won(),
            black_won(),
            draw(),
            white_won(),
            black_won(),
        ];
        let more_draws = vec![
            black_won(),
            draw(),
            draw(),
            draw(),
            draw(),
            white_won(),
            black_won(),
        ];
        let more_aborts = vec![
            black_won(),
            aborted(),
            white_won(),
            draw(),
            aborted(),
            aborted(),
        ];

        let (ww0, bw0, d0, a0) = results(&mut white_wins.iter());

        assert!(close(ww0, 0.5));
        assert!(close(bw0, 1. / 6.));
        assert!(close(d0, 1. / 6.));
        assert!(close(a0, 1. / 6.));

        let (ww1, bw1, d1, a1) = results(&mut black_wins.iter());

        assert!(close(ww1, 0.25));
        assert!(close(bw1, 0.375));
        assert!(close(d1, 0.25));
        assert!(close(a1, 0.125));

        let (ww2, bw2, d2, a2) = results(&mut more_draws.iter());

        assert!(close(ww2, 1. / 7.));
        assert!(close(bw2, 2. / 7.));
        assert!(close(d2, 4. / 7.));
        assert!(close(a2, 0.));

        let (ww3, bw3, d3, a3) = results(&mut more_aborts.iter());

        assert!(close(ww3, 1. / 6.));
        assert!(close(bw3, 1. / 6.));
        assert!(close(d3, 1. / 6.));
        assert!(close(a3, 0.5));
    }
}
