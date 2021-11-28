use io::Read;
use std::io;

use clap::{App, Arg, SubCommand};
use pgn_reader::BufferedReader;

use chesshound::{stats, AlgebraicMove, Game, GameParser, Move, MoveTree};

fn main() -> io::Result<()> {
    let matches = App::new("Chesshound")
        .version("0.1.0")
        .author("Gage C. <github.com/grchristensen>")
        .about("Chesshound is a CLI tool for finding patterns in sets of chess games")
        .subcommand(
            SubCommand::with_name("stats")
                .about("Takes PGN from standard input and gives statistics on games found")
                .arg(
                    Arg::with_name("branches")
                        .help("Show all moves that occur after this one in the game set")
                        .short("b")
                        .long("branches"),
                )
                .arg(
                    Arg::with_name("MOVES")
                        .help("Filters games by moves played")
                        .index(1)
                        .multiple(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("stats") {
        let pgn = io::stdin()
            .lock()
            .bytes()
            .map(|b| b.unwrap())
            .collect::<Vec<_>>();

        let moves: Vec<String> = if let Some(values) = matches.values_of("MOVES") {
            values.map(|move_| String::from(move_)).collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        let show_branches = matches.is_present("branches");

        println!("{}", run_stats(&pgn, moves, show_branches)?);
    }

    Ok(())
}

fn run_stats(pgn: &[u8], moves: Vec<String>, show_branches: bool) -> io::Result<String> {
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

    let branches: Vec<String> = if show_branches {
        let mut branches = match move_tree_view.branches() {
            Some(branches) => branches.map(|move_| move_.clone().to_algebraic()).collect(),
            None => Vec::new(),
        };

        branches.sort();

        branches
    } else {
        Vec::new()
    };

    let filtered_games = move_tree_view.iter().collect::<Vec<_>>();
    let game_count = filtered_games.len();

    let (white_win_rate, black_win_rate, draw_rate) =
        stats::results(&mut filtered_games.into_iter());
    let white_win_percent = white_win_rate * 100.;
    let black_win_percent = black_win_rate * 100.;
    let draw_percent = draw_rate * 100.;

    let mut output: String = format!(
        "{} games\nWhite Wins: {:.2}%\nBlack Wins: {:.2}%\nDraw: {:.2}%",
        game_count, white_win_percent, black_win_percent, draw_percent
    )
    .to_owned();

    if show_branches {
        let branches_output = if branches.len() > 0 {
            let mut branches_output = "Moves:".to_owned();

            for branch in branches {
                branches_output += &(" ".to_owned() + &branch);
            }

            branches_output
        } else {
            "No moves".to_owned()
        };

        output = output + "\n" + &branches_output;
    }

    Ok(output)
}
