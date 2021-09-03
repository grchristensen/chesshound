use io::Read;
use std::io;

use clap::{App, Arg, SubCommand};
use pgn_reader::BufferedReader;

use chesshound::{stats, AlgebraicMove, Move, Game, GameParser, MoveTree};

fn main() -> io::Result<()> {
    let matches = App::new("Chesshound")
        .version("0.1.0")
        .author("Gage C. <github.com/grchristensen>")
        .about("Chesshound is a CLI for finding patterns in sets of chess games")
        .subcommand(
            SubCommand::with_name("stats").about("Gives statistics on game sets")
            .arg(Arg::with_name("MOVES")
                 .help("Filters games by moves played")
                 .index(1)
                 .multiple(true)
            )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("stats") {
        let buffer = io::stdin()
            .lock()
            .bytes()
            .map(|b| b.unwrap())
            .collect::<Vec<_>>();

        let moves: Vec<String> = if let Some(values) = matches.values_of("MOVES") {
            values.map(|move_| String::from(move_)).collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        println!("{}", run_stats(&buffer, moves)?);
    }

    Ok(())
}

fn run_stats(pgn: &[u8], moves: Vec<String>) -> io::Result<String> {
    let mut reader = BufferedReader::new_cursor(&pgn[..]);

    fn read_game<R: Read>(
        reader: &mut BufferedReader<R>,
    ) -> io::Result<Option<Game<AlgebraicMove>>> {
        let mut game_parser = GameParser::new();
        let pgn_game = reader.read_game(&mut game_parser)?;

        Ok(pgn_game.map(|g| Game::<AlgebraicMove>::from(g)))
    }

    let mut games: Vec<Game<AlgebraicMove>> = Vec::new();

    while let Some(game) = read_game(&mut reader)? {
        games.push(game);
    }

    let move_tree = MoveTree::new(games);
    let mut move_tree_view = move_tree.view();

    for move_ in moves {
        move_tree_view = move_tree_view.with_next(&AlgebraicMove::from_algebraic(move_));
    }

    let filtered_games = move_tree_view.iter().collect::<Vec<_>>();
    let game_count = filtered_games.len();

    let (white_win_rate, black_win_rate, draw_rate) = stats::results(&mut filtered_games.into_iter());
    let white_win_percent = white_win_rate * 100.;
    let black_win_percent = black_win_rate * 100.;
    let draw_percent = draw_rate * 100.;

    Ok(format!(
        "{} games\nWhite Wins: {:.2}%\nBlack Wins: {:.2}%\nDraw: {:.2}%",
        game_count, white_win_percent, black_win_percent, draw_percent
    ))
}
