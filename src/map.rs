use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::{hash_map::Entry, HashMap};
use std::fmt;

const UP: usize = 0b1000;
const LEFT: usize = 0b0100;
const RIGHT: usize = 0b0010;
const DOWN: usize = 0b0001;

#[derive(Copy, Clone)]
pub struct WallJunction(usize);

impl WallJunction {
	fn set(&mut self, bit: usize, activate: bool) {
		if activate {
			self.0 |= bit;
		} else {
			self.0 &= !bit;
		}
	}
	fn is(&self, bit: usize) -> bool {
		self.0 & bit != 0
	}

	pub fn set_up(&mut self, activate: bool) {
		self.set(UP, activate)
	}
	pub fn is_up(&self) -> bool {
		self.is(UP)
	}
	pub fn set_left(&mut self, activate: bool) {
		self.set(LEFT, activate)
	}
	pub fn is_left(&self) -> bool {
		self.is(LEFT)
	}
	pub fn set_right(&mut self, activate: bool) {
		self.set(RIGHT, activate)
	}
	pub fn is_right(&self) -> bool {
		self.is(RIGHT)
	}
	pub fn set_down(&mut self, activate: bool) {
		self.set(DOWN, activate)
	}
	pub fn is_down(&self) -> bool {
		self.is(DOWN)
	}
}

impl Default for WallJunction {
	fn default() -> Self {
		WallJunction(0)
	}
}

impl From<WallJunction> for char {
	fn from(wj: WallJunction) -> Self {
		match wj.0 {
			b if b == UP => '╵',
			b if b == UP | LEFT => '┘',
			b if b == UP | LEFT | RIGHT => '┴',
			b if b == UP | LEFT | RIGHT | DOWN => '┼',
			b if b == UP | LEFT | DOWN => '┤',
			b if b == UP | RIGHT => '└',
			b if b == UP | RIGHT | DOWN => '├',
			b if b == UP | DOWN => '│',
			b if b == LEFT => '╴',
			b if b == LEFT | RIGHT => '─',
			b if b == LEFT | RIGHT | DOWN => '┬',
			b if b == LEFT | DOWN => '┐',
			b if b == RIGHT => '╶',
			b if b == RIGHT | DOWN => '┌',
			b if b == DOWN => '╷',
			_ => ' ',
		}
	}
}

impl fmt::Display for WallJunction {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", char::from(*self))
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
	Up,
	Left,
	Right,
	Down,
}

impl fmt::Display for Direction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Direction::Up => "↑",
				Direction::Left => "←",
				Direction::Right => "→",
				Direction::Down => "↓",
			}
		)
	}
}

const UPPER_LEFT: WallJunction = WallJunction(RIGHT | DOWN);
const UPPER_RIGHT: WallJunction = WallJunction(LEFT | DOWN);
const LOWER_LEFT: WallJunction = WallJunction(RIGHT | UP);
const LOWER_RIGHT: WallJunction = WallJunction(LEFT | UP);
const HORIZONTAL: WallJunction = WallJunction(LEFT | RIGHT);
const VERTICAL: WallJunction = WallJunction(UP | DOWN);

const DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Left, Direction::Right, Direction::Down];

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Position(
	/// Row
	pub usize,
	/// Column
	pub usize,
);

pub struct Map {
	pub rows: usize,
	pub columns: usize,
	map: Box<[bool]>,
}

impl Map {
	fn new_with_value(rows: usize, columns: usize, value: bool) -> Map {
		Map {
			rows,
			columns,
			map: vec![value; rows * 2 * columns - (rows + columns)].into_boxed_slice(),
		}
	}
	pub fn new(rows: usize, columns: usize) -> Map {
		Map::new_with_value(rows, columns, true)
	}
	pub fn new_empty(rows: usize, columns: usize) -> Map {
		Map::new_with_value(rows, columns, false)
	}

	pub fn generate_dfs<F, G>(rows: usize, columns: usize, start: Position, mut initial_peek: F, mut peek: G) -> Map
	where
		F: FnMut(&Map),
		G: FnMut(&Map, &Position, &Direction),
	{
		let mut map = Map::new(rows, columns);
		initial_peek(&map);
		let mut rng = thread_rng();

		let mut visited = HashSet::new();
		visited.insert(start);
		let mut to_visit = vec![start];

		let mut moved_positions = Vec::with_capacity(4);

		while let Some(next) = to_visit.pop() {
			moved_positions.clear();
			moved_positions.extend(
				DIRECTIONS
					.iter()
					.filter_map(|d| map.move_in_direction(&next, d).map(|m| (m, d)))
					.filter(|(m, _)| !visited.contains(&m)),
			);
			if moved_positions.len() > 1 {
				to_visit.push(next);
			}
			if let Some((moved, dir)) = moved_positions.choose(&mut rng) {
				map.set(&next, dir, false);
				to_visit.push(*moved);
				visited.insert(*moved);
				peek(&map, &next, &dir);
			}
		}

		map
	}

	pub fn generate_tree<F, G>(rows: usize, columns: usize, mut initial_peek: F, mut peek: G) -> Map
	where
		F: FnMut(&Map),
		G: FnMut(&Map, &Position, &Direction),
	{
		let mut map = Map::new(rows, columns);
		initial_peek(&map);

		for c in 1..map.columns {
			map.set_left(&Position(0, c), false);

			peek(&map, &Position(0, c), &Direction::Left);
		}
		for r in 1..map.rows {
			map.set_above(&Position(r, 0), false);

			peek(&map, &Position(r, 0), &Direction::Up);
		}

		for r in 1..map.rows {
			for c in 1..map.columns {
				if rand::random() {
					map.set_left(&Position(r, c), false);
					peek(&map, &Position(r, c), &Direction::Left);
				} else {
					map.set_above(&Position(r, c), false);
					peek(&map, &Position(r, c), &Direction::Up);
				}
			}
		}

		map
	}

	pub fn generate_prim<F, G>(rows: usize, columns: usize, start: Position, mut initial_peek: F, mut peek: G) -> Map
	where
		F: FnMut(&Map),
		G: FnMut(&Map, &Position, &Direction),
	{
		let mut map = Map::new(rows, columns);
		initial_peek(&map);
		let mut rng = thread_rng();

		let mut visited = HashSet::new();
		visited.insert(start);
		let mut walls = map.walls_around(&start);

		while !walls.is_empty() {
			let index = rng.gen_range(0, walls.len());
			let (from, dir) = walls.remove(index);
			if let Some(to) = map.move_in_direction(&from, &dir) {
				if !visited.contains(&to) {
					map.set(&from, &dir, false);

					visited.insert(to);
					walls.append(&mut map.walls_around(&to));

					peek(&map, &from, &dir);
				}
			}
		}

		map
	}

	pub fn generate_ab<F, G>(rows: usize, columns: usize, start: Position, mut initial_peek: F, mut peek: G) -> Map
	where
		F: FnMut(&Map),
		G: FnMut(&Map, &Position, &Direction),
	{
		let mut map = Map::new(rows, columns);
		initial_peek(&map);
		let mut rng = thread_rng();

		let mut visited = HashSet::new();
		visited.insert(start);
		let mut current = start;

		let mut moved_positions = Vec::with_capacity(4);
		while visited.len() < rows * columns {
			moved_positions.clear();
			moved_positions.extend(
				DIRECTIONS
					.iter()
					.filter_map(|d| map.move_in_direction(&current, d).map(|m| (m, d))),
			);
			if let Some(moved) = moved_positions.choose(&mut rng) {
				if !visited.contains(&moved.0) {
					map.set(&current, moved.1, false);
					peek(&map, &current, moved.1);
					visited.insert(moved.0);
				}
				current = moved.0;
			}
		}

		map
	}

	pub fn generate_div<F, G>(rows: usize, columns: usize, mut initial_peek: F, mut peek: G) -> Map
	where
		F: FnMut(&Map),
		G: FnMut(&Map, &Position, &Direction),
	{
		fn recurse_vertical<R, G>(map: &mut Map, rng: &mut R, upper_left: Position, lower_right: Position, peek: &mut G)
		where
			R: Rng + ?Sized,
			G: FnMut(&Map, &Position, &Direction),
		{
			if upper_left.1 < lower_right.1 {
				let div = rng.gen_range(upper_left.1, lower_right.1);
				let passage = rng.gen_range(upper_left.0, lower_right.0 + 1);

				for r in upper_left.0..(lower_right.0 + 1) {
					if r != passage {
						map.set_right(&Position(r, div), true);
						peek(&map, &Position(r, div), &Direction::Right);
					}
				}

				if upper_left.0 >= lower_right.0 {
					recurse_vertical(map, rng, upper_left, Position(lower_right.0, div), peek);
					recurse_vertical(map, rng, Position(upper_left.0, div + 1), lower_right, peek);
				} else {
					recurse_horizontal(map, rng, upper_left, Position(lower_right.0, div), peek);
					recurse_horizontal(map, rng, Position(upper_left.0, div + 1), lower_right, peek);
				}
			}
		}
		fn recurse_horizontal<R, G>(
			map: &mut Map,
			rng: &mut R,
			upper_left: Position,
			lower_right: Position,
			peek: &mut G,
		) where
			R: Rng + ?Sized,
			G: FnMut(&Map, &Position, &Direction),
		{
			if upper_left.0 < lower_right.0 {
				let div = rng.gen_range(upper_left.0, lower_right.0);
				let passage = rng.gen_range(upper_left.1, lower_right.1 + 1);

				for c in upper_left.1..(lower_right.1 + 1) {
					if c != passage {
						map.set_below(&Position(div, c), true);
						peek(&map, &Position(div, c), &Direction::Down);
					}
				}

				if upper_left.1 >= lower_right.1 {
					recurse_horizontal(map, rng, upper_left, Position(div, lower_right.1), peek);
					recurse_horizontal(map, rng, Position(div + 1, upper_left.1), lower_right, peek);
				} else {
					recurse_vertical(map, rng, upper_left, Position(div, lower_right.1), peek);
					recurse_vertical(map, rng, Position(div + 1, upper_left.1), lower_right, peek);
				}
			}
		}

		let mut map = Map::new_empty(rows, columns);
		initial_peek(&map);
		let mut rng = thread_rng();

		let upper_left = Position(0, 0);
		let lower_right = Position(map.rows - 1, map.columns - 1);
		recurse_vertical(&mut map, &mut rng, upper_left, lower_right, &mut peek);

		map
	}

	pub fn generate_wilson<F, G>(rows: usize, columns: usize, start: Position, mut initial_peek: F, mut peek: G) -> Map
	where
		F: FnMut(&Map),
		G: FnMut(&Map, &Position, &Direction),
	{
		let mut map = Map::new(rows, columns);
		initial_peek(&map);
		let mut rng = thread_rng();

		let mut in_map = HashSet::new();
		in_map.insert(start);
		let mut unvisited: Vec<_> = (0..rows)
			.flat_map(|r| (0..columns).filter_map(move |c| Some(Position(r, c)).filter(|p| p != &start)))
			.collect();

		let mut path: Vec<(Position, Direction)> = Vec::new();
		let mut moved_positions = Vec::with_capacity(4);
		while !unvisited.is_empty() {
			let mut current = unvisited[rng.gen_range(0, unvisited.len())];

			while !in_map.contains(&current) {
				moved_positions.clear();
				moved_positions.extend(
					DIRECTIONS
						.iter()
						.filter_map(|d| map.move_in_direction(&current, d).map(|m| (m, d))),
				);
				if let Some((next, direction)) = moved_positions.choose(&mut rng) {
					if let Some(index) = path.iter().position(|p| &p.0 == next) {
						for (p, d) in path.drain(index..).rev() {
							map.set(&p, &d, true);
							peek(&map, &p, &d);
						}
					} else {
						map.set(&current, direction, false);
						peek(&map, &current, direction);
						path.push((current, **direction));
					}
					current = *next;
				}
			}

			for (p, _) in path.drain(..) {
				in_map.insert(p);
				unvisited.swap_remove(unvisited.iter().position(|u| u == &p).unwrap());
			}
		}

		map
	}

	pub fn set_above(&mut self, pos: &Position, closed: bool) {
		self.set_below(&Position(pos.0 - 1, pos.1), closed);
	}
	pub fn is_above(&self, pos: &Position) -> bool {
		self.is_below(&Position(pos.0 - 1, pos.1))
	}
	pub fn set_left(&mut self, pos: &Position, closed: bool) {
		self.set_right(&Position(pos.0, pos.1 - 1), closed);
	}
	pub fn is_left(&self, pos: &Position) -> bool {
		self.is_right(&Position(pos.0, pos.1 - 1))
	}
	pub fn set_right(&mut self, pos: &Position, closed: bool) {
		assert!(pos.0 < self.rows && pos.1 < self.columns - 1);

		self.map[(self.rows - 1) * self.columns + pos.0 * (self.columns - 1) + pos.1] = closed;
	}
	pub fn is_right(&self, pos: &Position) -> bool {
		assert!(pos.0 < self.rows && pos.1 < self.columns - 1);

		self.map[(self.rows - 1) * self.columns + pos.0 * (self.columns - 1) + pos.1]
	}
	pub fn set_below(&mut self, pos: &Position, closed: bool) {
		assert!(pos.0 < self.rows - 1 && pos.1 < self.columns);

		self.map[pos.0 * self.columns + pos.1] = closed;
	}
	pub fn is_below(&self, pos: &Position) -> bool {
		assert!(pos.0 < self.rows - 1 && pos.1 < self.columns);

		self.map[pos.0 * self.columns + pos.1]
	}

	pub fn set(&mut self, pos: &Position, dir: &Direction, closed: bool) {
		match dir {
			Direction::Up => self.set_above(pos, closed),
			Direction::Left => self.set_left(pos, closed),
			Direction::Right => self.set_right(pos, closed),
			Direction::Down => self.set_below(pos, closed),
		};
	}
	pub fn is(&self, pos: &Position, dir: &Direction) -> Option<bool> {
		match dir {
			Direction::Up if 0 < pos.0 && pos.0 < self.rows && pos.1 < self.columns => Some(self.is_above(pos)),
			Direction::Left if pos.0 < self.rows && 0 < pos.1 && pos.1 < self.columns => Some(self.is_left(pos)),
			Direction::Right if pos.0 < self.rows && pos.1 < self.columns - 1 => Some(self.is_right(pos)),
			Direction::Down if pos.0 < self.rows - 1 && pos.1 < self.columns => Some(self.is_below(pos)),
			_ => None,
		}
	}

	fn move_in_direction(&self, current: &Position, dir: &Direction) -> Option<Position> {
		match dir {
			Direction::Up if current.0 > 0 => Some(Position(current.0 - 1, current.1)),
			Direction::Left if current.1 > 0 => Some(Position(current.0, current.1 - 1)),
			Direction::Right if current.1 < self.columns - 1 => Some(Position(current.0, current.1 + 1)),
			Direction::Down if current.0 < self.rows - 1 => Some(Position(current.0 + 1, current.1)),
			_ => None,
		}
	}

	fn walls_around(&self, pos: &Position) -> Vec<(Position, Direction)> {
		DIRECTIONS
			.iter()
			.filter_map(|dir| {
				if self.is(pos, dir) == Some(true) {
					return Some((*pos, *dir));
				}
				None
			})
			.collect()
	}

	fn possible_moves_for(&self, pos: &Position) -> Vec<Position> {
		DIRECTIONS
			.iter()
			.filter_map(|dir| {
				if self.is(pos, dir) == Some(false) {
					return self.move_in_direction(pos, dir);
				}
				None
			})
			.collect()
	}

	pub fn solve(&self, from: Position, to: Position) -> Option<Vec<Direction>> {
		assert!(from.0 < self.rows && from.1 < self.columns);
		assert!(to.0 < self.rows && to.1 < self.columns);

		if from == to {
			return Some(Vec::new());
		}

		let mut from_to = HashMap::new();
		from_to.insert(from, None);
		let mut to_visit = VecDeque::new();
		to_visit.push_back(from);

		while let Some(next) = to_visit.pop_front() {
			for moved in self.possible_moves_for(&next) {
				if let Entry::Vacant(e) = from_to.entry(moved) {
					e.insert(Some(next));
					if moved == to {
						return Some(build_path(from_to, to));
					}
					to_visit.push_back(moved);
				}
			}
		}

		None
	}

	pub fn get_chars(&self, pos: &Position, dir: &Direction) -> (char, char) {
		if dir == &Direction::Left || dir == &Direction::Right {
			let mut above = WallJunction::default();
			let mut below = WallJunction::default();

			if self.is(pos, dir).unwrap_or(true) {
				above.set_down(true);
				below.set_up(true);
			}

			if pos.0 != 0 && self.is(&Position(pos.0 - 1, pos.1), dir).unwrap_or(true) {
				above.set_up(true);
			}
			if pos.0 != self.rows - 1 && self.is(&Position(pos.0 + 1, pos.1), dir).unwrap_or(true) {
				below.set_down(true);
			}

			if dir == &Direction::Left {
				if pos.0 == 0 || self.is_above(pos) {
					above.set_right(true);
				}
				if pos.0 == self.rows - 1 || self.is_below(pos) {
					below.set_right(true);
				}
				if pos.1 != 0 {
					if pos.0 == 0 || self.is_above(&Position(pos.0, pos.1 - 1)) {
						above.set_left(true);
					}
					if pos.0 == self.rows - 1 || self.is_below(&Position(pos.0, pos.1 - 1)) {
						below.set_left(true);
					}
				}
			} else {
				if pos.0 == 0 || self.is_above(pos) {
					above.set_left(true);
				}
				if pos.0 == self.rows - 1 || self.is_below(pos) {
					below.set_left(true);
				}
				if pos.1 != self.columns - 1 {
					if pos.0 == 0 || self.is_above(&Position(pos.0, pos.1 + 1)) {
						above.set_right(true);
					}
					if pos.0 == self.rows - 1 || self.is_below(&Position(pos.0, pos.1 + 1)) {
						below.set_right(true);
					}
				}
			}

			(char::from(above), char::from(below))
		} else {
			let mut left = WallJunction::default();
			let mut right = WallJunction::default();

			if self.is(pos, dir).unwrap_or(true) {
				left.set_right(true);
				right.set_left(true);
			}

			if pos.1 != 0 && self.is(&Position(pos.0, pos.1 - 1), dir).unwrap_or(true) {
				left.set_left(true);
			}
			if pos.1 != self.columns - 1 && self.is(&Position(pos.0, pos.1 + 1), dir).unwrap_or(true) {
				right.set_right(true);
			}

			if dir == &Direction::Up {
				if pos.1 == 0 || self.is_left(pos) {
					left.set_down(true);
				}
				if pos.1 == self.columns - 1 || self.is_right(pos) {
					right.set_down(true);
				}
				if pos.0 != 0 {
					if pos.1 == 0 || self.is_left(&Position(pos.0 - 1, pos.1)) {
						left.set_up(true);
					}
					if pos.1 == self.columns - 1 || self.is_right(&Position(pos.0 - 1, pos.1)) {
						right.set_up(true);
					}
				}
			} else {
				if pos.1 == 0 || self.is_left(pos) {
					left.set_up(true);
				}
				if pos.1 == self.columns - 1 || self.is_right(pos) {
					right.set_up(true);
				}
				if pos.0 != self.rows - 1 {
					if pos.1 == 0 || self.is_left(&Position(pos.0 + 1, pos.1)) {
						left.set_down(true);
					}
					if pos.1 == self.columns - 1 || self.is_right(&Position(pos.0 + 1, pos.1)) {
						right.set_down(true);
					}
				}
			}

			(char::from(left), char::from(right))
		}
	}
}

fn build_path(mut from_to: HashMap<Position, Option<Position>>, to: Position) -> Vec<Direction> {
	if let Some(Some(from)) = from_to.remove(&to) {
		let mut part = build_path(from_to, from);
		part.push(
			match ((from.0 as isize) - (to.0 as isize), (from.1 as isize) - (to.1 as isize)) {
				(1, 0) => Direction::Up,
				(0, 1) => Direction::Left,
				(0, -1) => Direction::Right,
				(-1, 0) => Direction::Down,
				(_, _) => panic!(),
			},
		);
		part
	} else {
		Vec::new()
	}
}

impl fmt::Display for Map {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut above;
		let mut below = Vec::with_capacity(self.columns + 1);

		below.push(UPPER_LEFT);
		below.append(&mut vec![HORIZONTAL; self.columns - 1]);
		below.push(UPPER_RIGHT);

		for r in 0..(self.rows - 1) {
			above = below;
			below = vec![WallJunction::default(); self.columns + 1];
			below[0] = VERTICAL;
			below[self.columns] = VERTICAL;

			for c in 0..(self.columns - 1) {
				above[c + 1].set_down(self.is_right(&Position(r, c)));
				below[c + 1].set_up(self.is_right(&Position(r, c)));
			}

			for c in 0..self.columns {
				below[c].set_right(self.is_below(&Position(r, c)));
				below[c + 1].set_left(self.is_below(&Position(r, c)));
			}

			writeln!(f, "{}", above.into_iter().map(|j| format!("{}", j)).collect::<String>())?;
		}

		above = below;
		below = vec![HORIZONTAL; self.columns + 1];
		below[0] = LOWER_LEFT;
		below[self.columns] = LOWER_RIGHT;
		for c in 0..(self.columns - 1) {
			above[c + 1].set_down(self.is_right(&Position(self.rows - 1, c)));
			below[c + 1].set_up(self.is_right(&Position(self.rows - 1, c)));
		}

		writeln!(f, "{}", above.into_iter().map(|j| format!("{}", j)).collect::<String>())?;
		write!(f, "{}", below.into_iter().map(|j| format!("{}", j)).collect::<String>())
	}
}
