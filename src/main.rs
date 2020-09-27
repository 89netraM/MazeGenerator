extern crate crossterm;
use crossterm::{cursor, ExecutableCommand};
use std::io::{stdout, Write};

use std::{thread, time::Duration};

mod map;
use map::Map;

fn main() {
	let mut stdout = stdout();
	let mut move_height = 0;
	stdout
		.execute(cursor::Hide)
		.expect("Could not hide cursor.");
	let map = Map::generate_with_peek(15, 30, (0, 0), &mut move |map| {
		if move_height > 0 {
			stdout
				.execute(cursor::MoveUp(move_height))
				.expect("Could not move cursor.");
		}
		writeln!(stdout, "{}", map).expect("Could not write.");
		move_height = (map.rows as u16) + 1;

		thread::sleep(Duration::from_millis(50));
	});

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
