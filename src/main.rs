mod map;
use map::Direction;
use map::Map;

fn main() {
	let mut map = Map::new(5, 5);
	map.set(0, 0, Direction::Down, false);
	map.set(1, 0, Direction::Right, false);
	map.set(1, 1, Direction::Right, false);
	map.set(1, 2, Direction::Down, false);
	map.set(2, 2, Direction::Down, false);
	map.set(3, 2, Direction::Right, false);
	map.set(3, 3, Direction::Down, false);
	map.set(4, 3, Direction::Right, false);

	map.set(3, 3, Direction::Up, false);
	map.set(2, 3, Direction::Up, false);
	map.set(1, 3, Direction::Up, false);
	map.set(0, 3, Direction::Left, false);
	map.set(0, 2, Direction::Left, false);

	map.set(1, 3, Direction::Right, false);
	map.set(1, 4, Direction::Up, false);
	map.set(1, 4, Direction::Down, false);
	map.set(2, 4, Direction::Down, false);

	map.set(3, 2, Direction::Left, false);
	map.set(3, 1, Direction::Left, false);
	map.set(3, 0, Direction::Up, false);
	map.set(2, 0, Direction::Right, false);

	map.set(3, 0, Direction::Down, false);
	map.set(4, 0, Direction::Right, false);
	map.set(4, 1, Direction::Right, false);

	println!("Map: \n{}", map);

	if let Some(path) = map.solve((0, 0), (4, 4)) {
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
