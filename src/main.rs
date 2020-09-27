mod map;
use map::Map;

fn main() {
	let map = Map::generate(30, 15, (0, 0));

	println!("Map: \n{}", map);

	if let Some(path) = map.solve((0, 0), (14, 29)) {
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
