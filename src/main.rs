use crossterm::{cursor, ExecutableCommand, QueueableCommand};
use std::io::{stdout, Write};

use clap::{App, Arg, ArgGroup, ArgMatches};
use std::str::FromStr;

use std::{thread, time::Duration};

mod map;
use map::Direction;
use map::Map;
use map::Position;

fn main() {
	let matches = App::new("Maze Generator")
		.arg(
			Arg::with_name("ROWS")
				.long("rows")
				.default_value("5")
				.validator(check_arg_is_number)
				.help("Number of rows of the generated map")
				.display_order(0),
		)
		.arg(
			Arg::with_name("COLUMNS")
				.long("columns")
				.default_value("5")
				.validator(check_arg_is_number)
				.help("Number of columns of the generated map")
				.display_order(1),
		)
		.arg(
			Arg::with_name("START_ROW")
				.long("start_row")
				.default_value("0")
				.validator(check_arg_is_number)
				.help("The row to start generating from")
				.display_order(2),
		)
		.arg(
			Arg::with_name("START_COLUMN")
				.long("start_column")
				.default_value("0")
				.validator(check_arg_is_number)
				.help("The column to start generating from")
				.display_order(3),
		)
		.arg(
			Arg::with_name("DELAY")
				.long("delay")
				.default_value("50")
				.validator(check_arg_is_number)
				.help("The ms delay between steps")
				.display_order(4),
		)
		.arg(
			Arg::with_name("DFS")
				.long("dfs")
				.help("Use the depth first search algorithm for maze generation [default]")
				.display_order(5),
		)
		.arg(
			Arg::with_name("TREE")
				.long("tree")
				.help("Use the binary tree maze algorithm for maze generation")
				.display_order(6),
		)
		.arg(
			Arg::with_name("PRIM")
				.long("prim")
				.help("Use Prim's algorithm for maze generation")
				.display_order(7),
		)
		.arg(
			Arg::with_name("AB")
				.long("ab")
				.help("Use the Aldous-Broder algorithm for maze generation")
				.display_order(8),
		)
		.arg(
			Arg::with_name("DIV")
				.long("div")
				.help("Use the recursive division method for maze generation")
				.display_order(9),
		)
		.group(ArgGroup::with_name("ALGORITHM").args(&[
			"DFS",
			"TREE",
			"PRIM",
			"AB",
			"DIV",
		]))
		.get_matches();

	let rows = get_arg_as_t(&matches, "ROWS");
	let columns = get_arg_as_t(&matches, "COLUMNS");
	let start_pos = Position(
		get_arg_as_t(&matches, "START_ROW"),
		get_arg_as_t(&matches, "START_COLUMN"),
	);
	let delay = get_arg_as_t(&matches, "DELAY");

	let mut stdout = stdout();
	stdout.execute(cursor::Hide).expect("Could not hide cursor.");
	let initial_peek_fn = |map: &Map| println!("{}", map);
	let peek_fn = |map: &Map, pos: &Position, dir: &Direction| {
		let chars = map.get_chars(pos, dir);
		let rows = (map.rows - pos.0) as u16 + if dir == &Direction::Up { 1 } else { 0 };
		let columns = pos.1 as u16 + if dir == &Direction::Right { 1 } else { 0 };

		if dir == &Direction::Left || dir == &Direction::Right {
			stdout.queue(cursor::MoveUp(rows + 1)).expect("Could not move cursor.");
			if columns > 0 {
				stdout
					.queue(cursor::MoveRight(columns))
					.expect("Could not move cursor.");
			}
			stdout.write_fmt(format_args!("{}", chars.0)).expect("Could not write.");
			stdout.queue(cursor::MoveDown(1)).expect("Could not move cursor.");
			stdout.queue(cursor::MoveLeft(1)).expect("Could not move cursor.");
			stdout.write_fmt(format_args!("{}", chars.1)).expect("Could not write.");
			stdout
				.queue(cursor::MoveLeft(columns + 1))
				.expect("Could not move cursor.");
			if rows > 0 {
				stdout.queue(cursor::MoveDown(rows)).expect("Could not move cursor.");
			}
		} else {
			if rows > 0 {
				stdout.queue(cursor::MoveUp(rows)).expect("Could not move cursor.");
			}
			if columns > 0 {
				stdout
					.queue(cursor::MoveRight(columns))
					.expect("Could not move cursor.");
			}
			stdout
				.write_fmt(format_args!("{}{}", chars.0, chars.1))
				.expect("Could not write.");
			stdout
				.queue(cursor::MoveLeft(columns + 2))
				.expect("Could not move cursor.");
			if rows > 0 {
				stdout.queue(cursor::MoveDown(rows)).expect("Could not move cursor.");
			}
		}
		stdout.flush().expect("Could not flush.");

		if delay > 0 {
			thread::sleep(Duration::from_millis(delay));
		}
	};
	let map = if matches.is_present("TREE") {
		Map::generate_tree(rows, columns, initial_peek_fn, peek_fn)
	} else if matches.is_present("PRIM") {
		Map::generate_prim(rows, columns, start_pos, initial_peek_fn, peek_fn)
	} else if matches.is_present("AB") {
		Map::generate_ab(rows, columns, start_pos, initial_peek_fn, peek_fn)
	} else if matches.is_present("DIV") {
		Map::generate_div(rows, columns, initial_peek_fn, peek_fn)
	} else {
		Map::generate_dfs(rows, columns, start_pos, initial_peek_fn, peek_fn)
	};
	stdout.execute(cursor::Show).expect("Could not show cursor.");

	if let Some(path) = map.solve(Position(0, 0), Position(map.rows - 1, map.columns - 1)) {
		println!(
			"Path: {}",
			path.into_iter().map(|d| format!("{}", d)).collect::<String>()
		);
	} else {
		println!("No path through maze");
	}
}

fn check_arg_is_number(s: String) -> Result<(), String> {
	if usize::from_str(&s).is_ok() {
		Ok(())
	} else {
		Err("Must be a number".to_string())
	}
}

fn get_arg_as_t<T: FromStr>(matches: &ArgMatches, name: &str) -> T {
	if let Some(s) = matches.value_of(name) {
		if let Ok(v) = T::from_str(s) {
			return v;
		}
	}

	panic!();
}
