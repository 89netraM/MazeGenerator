extern crate crossterm;
use crossterm::{cursor, ExecutableCommand};
use std::io::{stdout, Write};

extern crate clap;
use clap::{App, Arg, ArgGroup};
use std::str::FromStr;

use std::{thread, time::Duration};

mod map;
use map::Map;

fn main() {
	let matches = App::new("Maze Generator")
		.arg(Arg::with_name("ROWS")
			.long("rows")
			.default_value("5")
			.validator(|s| usize::from_str(&s).map(|_| ()).or(Err("Must be a number".to_string())))
			.help("Number of rows of the generated map")
			.display_order(0),
		)
		.arg(Arg::with_name("COLUMNS")
			.long("columns")
			.default_value("5")
			.validator(|s| usize::from_str(&s).map(|_| ()).or(Err("Must be a number".to_string())))
			.help("Number of columns of the generated map")
			.display_order(1),
		)
		.arg(Arg::with_name("START_ROW")
			.long("start_row")
			.default_value("0")
			.validator(|s| usize::from_str(&s).map(|_| ()).or(Err("Must be a number".to_string())))
			.help("The row to start generating from")
			.display_order(2),
		)
		.arg(Arg::with_name("START_COLUMN")
			.long("start_column")
			.default_value("0")
			.validator(|s| usize::from_str(&s).map(|_| ()).or(Err("Must be a number".to_string())))
			.help("The column to start generating from")
			.display_order(3),
		)
		.arg(Arg::with_name("DELAY")
			.long("delay")
			.default_value("50")
			.validator(|s| usize::from_str(&s).map(|_| ()).or(Err("Must be a number".to_string())))
			.help("The ms delay between steps")
			.display_order(4),
		)
		.arg(Arg::with_name("DFS")
			.long("dfs")
			.help("Use the depth first search algorithm for maze generation [default]")
			.display_order(5)
		)
		.arg(Arg::with_name("AB")
			.long("ab")
			.help("Use the Aldous-Broder algorithm for maze generation")
			.display_order(6)
		)
		.group(ArgGroup::with_name("ALGORITHM").args(&[
			"DFS",
			"AB",
		]))
		.get_matches();

	let rows = matches.value_of("ROWS").map(usize::from_str).unwrap().unwrap();
	let columns = matches.value_of("COLUMNS").map(usize::from_str).unwrap().unwrap();
	let start_row = matches.value_of("START_ROW").map(usize::from_str).unwrap().unwrap();
	let start_column = matches.value_of("START_COLUMN").map(usize::from_str).unwrap().unwrap();
	let delay = matches.value_of("DELAY").map(u64::from_str).unwrap().unwrap();

	let mut stdout = stdout();
	let mut move_height = 0;
	stdout
		.execute(cursor::Hide)
		.expect("Could not hide cursor.");
	let peek_fn = |map: &Map| {
		if move_height > 0 {
			stdout
				.execute(cursor::MoveUp(move_height))
				.expect("Could not move cursor.");
		}
		writeln!(stdout, "{}", map).expect("Could not write.");
		move_height = (map.rows as u16) + 1;

		if delay > 0 {
			thread::sleep(Duration::from_millis(delay));
		}
	};
	let map = if matches.is_present("AB") {
		Map::generate_ab_with_peek(rows, columns, peek_fn)
	} else {
		Map::generate_dfs_with_peek(rows, columns, (start_row, start_column), peek_fn)
	};
	stdout
		.execute(cursor::Show)
		.expect("Could not show cursor.");

	if let Some(path) = map.solve((0, 0), (map.rows - 1, map.columns - 1)) {
		println!(
			"Path: {}",
			path.into_iter()
				.map(|d| format!("{}", d))
				.collect::<Vec<String>>()
				.join(" ")
		);
	} else {
		println!("No path through maze");
	}
}
