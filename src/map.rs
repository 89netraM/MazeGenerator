extern crate rand;
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
	pub fn new() -> WallJunction {
		WallJunction(0)
	}

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
		write!(
			f,
			"{}",
			match self.0 {
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
		)
	}
}

#[derive(Copy, Clone)]
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
				Direction::Up => "⬆",
				Direction::Left => "⬅",
				Direction::Right => "➡",
				Direction::Down => "⬇",
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

const DIRECTIONS: [Direction; 4] = [
	Direction::Up,
	Direction::Left,
	Direction::Right,
	Direction::Down,
];

pub struct Map {
	width: usize,
	height: usize,
	map: Vec<Vec<bool>>,
}

impl Map {
	pub fn new(width: usize, height: usize) -> Map {
		let mut map = Vec::with_capacity(height * 2 - 1);

		for r in 0..(height * 2 - 1) {
			if r % 2 == 0 {
				map.push(vec![true; width - 1]);
			} else {
				map.push(vec![true; width]);
			}
		}

		Map { width, height, map }
	}

	pub fn generate(width: usize, height: usize, start: (usize, usize)) -> Map {
		Map::generate_with_peek(width, height, start, &mut |_| {})
	}
	pub fn generate_with_peek<F>(
		width: usize,
		height: usize,
		start: (usize, usize),
		peek: &mut F,
	) -> Map
	where
		F: FnMut(&Map),
	{
		let mut map = Map::new(width, height);
		map.make_path_with_peek(start, &mut HashSet::new(), &mut thread_rng(), peek);
		map
	}

	pub fn set_above(&mut self, r: usize, c: usize, closed: bool) {
		assert!(0 < r && r < self.height && c < self.width);

		self.map[(r * 2) - 1][c] = closed;
	}
	pub fn is_above(&self, r: usize, c: usize) -> bool {
		assert!(0 < r && r < self.height && c < self.width);

		self.map[(r * 2) - 1][c]
	}
	pub fn set_left(&mut self, r: usize, c: usize, closed: bool) {
		assert!(r < self.height && 0 < c && c < self.width);

		self.map[r * 2][c - 1] = closed;
	}
	pub fn is_left(&self, r: usize, c: usize) -> bool {
		assert!(r < self.height && 0 < c && c < self.width);

		self.map[r * 2][c - 1]
	}
	pub fn set_right(&mut self, r: usize, c: usize, closed: bool) {
		assert!(r < self.height && c < self.width - 1);

		self.map[r * 2][c] = closed;
	}
	pub fn is_right(&self, r: usize, c: usize) -> bool {
		assert!(r < self.height && c < self.width - 1);

		self.map[r * 2][c]
	}
	pub fn set_below(&mut self, r: usize, c: usize, closed: bool) {
		assert!(r < self.height - 1 && c < self.width);

		self.map[(r * 2) + 1][c] = closed;
	}
	pub fn is_below(&self, r: usize, c: usize) -> bool {
		assert!(r < self.height - 1 && c < self.width);

		self.map[(r * 2) + 1][c]
	}

	pub fn set(&mut self, r: usize, c: usize, dir: &Direction, closed: bool) {
		match dir {
			Direction::Up => self.set_above(r, c, closed),
			Direction::Left => self.set_left(r, c, closed),
			Direction::Right => self.set_right(r, c, closed),
			Direction::Down => self.set_below(r, c, closed),
		};
	}
	pub fn is(&self, r: usize, c: usize, dir: &Direction) -> bool {
		match dir {
			Direction::Up if 0 < r && r < self.height && c < self.width => self.is_above(r, c),
			Direction::Left if r < self.height && 0 < c && c < self.width => self.is_left(r, c),
			Direction::Right if r < self.height && c < self.width - 1 => self.is_right(r, c),
			Direction::Down if r < self.height - 1 && c < self.width => self.is_below(r, c),
			_ => true,
		}
	}

	fn move_in_direction(
		&self,
		current: &(usize, usize),
		dir: &Direction,
	) -> Option<(usize, usize)> {
		match dir {
			Direction::Up if current.0 > 0 => Some((current.0 - 1, current.1)),
			Direction::Left if current.1 > 0 => Some((current.0, current.1 - 1)),
			Direction::Right if current.1 < self.width - 1 => Some((current.0, current.1 + 1)),
			Direction::Down if current.0 < self.height - 1 => Some((current.0 + 1, current.1)),
			_ => None,
		}
	}

	fn possible_moves_for(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
		DIRECTIONS
			.iter()
			.filter_map(|dir| {
				if !self.is(pos.0, pos.1, dir) {
					return self.move_in_direction(pos, dir);
				}
				None
			})
			.collect()
	}

	pub fn solve(&self, from: (usize, usize), to: (usize, usize)) -> Option<Vec<Direction>> {
		assert!(from.0 < self.height && from.1 < self.width);
		assert!(to.0 < self.height && to.1 < self.width);

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

	fn make_path_with_peek<R, F>(
		&mut self,
		start: (usize, usize),
		taken: &mut HashSet<(usize, usize)>,
		rng: &mut R,
		peek: &mut F,
	) where
		R: Rng + ?Sized,
		F: FnMut(&Map),
	{
		taken.insert(start);

		let mut dirs = DIRECTIONS.clone();
		dirs.shuffle(rng);
		for dir in &dirs {
			if let Some(moved) = self.move_in_direction(&start, dir) {
				if taken.insert(moved) {
					self.set(start.0, start.1, dir, false);
					peek(&self);

					self.make_path_with_peek(moved, taken, rng, peek);
				}
			}
		}
	}
}

fn build_path(
	mut from_to: HashMap<(usize, usize), Option<(usize, usize)>>,
	to: (usize, usize),
) -> Vec<Direction> {
	if let Some(Some(from)) = from_to.remove(&to) {
		let mut part = build_path(from_to, from);
		part.push(
			match (
				(from.0 as isize) - (to.0 as isize),
				(from.1 as isize) - (to.1 as isize),
			) {
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
		let mut below = Vec::with_capacity(self.width + 1);

		below.push(UPPER_LEFT);
		below.append(&mut vec![HORIZONTAL; self.width - 1]);
		below.push(UPPER_RIGHT);

		for r in 0..(self.height - 1) {
			above = below;
			below = vec![WallJunction::new(); self.width + 1];
			below[0] = VERTICAL;
			below[self.width] = VERTICAL;

			for c in 0..(self.width - 1) {
				above[c + 1].set_down(self.map[r * 2][c]);
				below[c + 1].set_up(self.map[r * 2][c]);
			}

			for c in 0..self.width {
				below[c].set_right(self.map[r * 2 + 1][c]);
				below[c + 1].set_left(self.map[r * 2 + 1][c]);
			}

			writeln!(
				f,
				"{}",
				above
					.into_iter()
					.map(|j| format!("{}", j))
					.collect::<String>()
			)?;
		}

		above = below;
		below = vec![HORIZONTAL; self.width + 1];
		below[0] = LOWER_LEFT;
		below[self.width] = LOWER_RIGHT;
		for c in 0..(self.width - 1) {
			above[c + 1].set_down(self.map[self.height * 2 - 2][c]);
			below[c + 1].set_up(self.map[self.height * 2 - 2][c]);
		}

		writeln!(
			f,
			"{}",
			above
				.into_iter()
				.map(|j| format!("{}", j))
				.collect::<String>()
		)?;
		write!(
			f,
			"{}",
			below
				.into_iter()
				.map(|j| format!("{}", j))
				.collect::<String>()
		)
	}
}
