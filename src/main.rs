mod map;
use map::Map;

fn main() {
	let mut map = Map::new(5, 5);
	map.set_below(0, 0, false);
	map.set_right(1, 0, false);
	map.set_right(1, 1, false);
	map.set_below(1, 2, false);
	map.set_below(2, 2, false);
	map.set_right(3, 2, false);
	map.set_below(3, 3, false);
	map.set_right(4, 3, false);

	map.set_above(3, 3, false);
	map.set_above(2, 3, false);
	map.set_above(1, 3, false);
	map.set_left(0, 3, false);
	map.set_left(0, 2, false);

	map.set_right(1, 3, false);
	map.set_above(1, 4, false);
	map.set_below(1, 4, false);
	map.set_below(2, 4, false);

	map.set_left(3, 2, false);
	map.set_left(3, 1, false);
	map.set_above(3, 0, false);
	map.set_right(2, 0, false);

	map.set_below(3, 0, false);
	map.set_right(4, 0, false);
	map.set_right(4, 1, false);

	println!("Map: \n{}", map);
}
