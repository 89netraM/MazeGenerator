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

	pub fn set_above(&mut self, r: usize, c: usize, closed: bool) {
		assert!(0 < r && r < self.height && c < self.width);

		self.map[(r * 2) - 1][c] = closed;
	}
	pub fn is_above(&mut self, r: usize, c: usize) -> bool {
		assert!(0 < r && r < self.height && c < self.width);

		self.map[(r * 2) - 1][c]
	}
	pub fn set_left(&mut self, r: usize, c: usize, closed: bool) {
		assert!(r < self.height && 0 < c && c < self.width);

		self.map[r * 2][c - 1] = closed;
	}
	pub fn is_left(&mut self, r: usize, c: usize) -> bool {
		assert!(r < self.height && 0 < c && c < self.width);

		self.map[r * 2][c - 1]
	}
	pub fn set_right(&mut self, r: usize, c: usize, closed: bool) {
		assert!(r < self.height && c < self.width - 1);

		self.map[r * 2][c] = closed;
	}
	pub fn is_right(&mut self, r: usize, c: usize) -> bool {
		assert!(r < self.height && c < self.width - 1);

		self.map[r * 2][c]
	}
	pub fn set_below(&mut self, r: usize, c: usize, closed: bool) {
		assert!(r < self.height - 1 && c < self.width);

		self.map[(r * 2) + 1][c] = closed;
	}
	pub fn is_below(&mut self, r: usize, c: usize) -> bool {
		assert!(r < self.height - 1 && c < self.width);

		self.map[(r * 2) + 1][c]
	}

	pub fn set(&mut self, r: usize, c: usize, dir: Direction, closed: bool) {
		match dir {
			Direction::Up => self.set_above(r, c, closed),
			Direction::Left => self.set_left(r, c, closed),
			Direction::Right => self.set_right(r, c, closed),
			Direction::Down => self.set_below(r, c, closed),
		};
	}
	pub fn is(&mut self, r: usize, c: usize, dir: Direction) -> bool {
		match dir {
			Direction::Up => self.is_above(r, c),
			Direction::Left => self.is_left(r, c),
			Direction::Right => self.is_right(r, c),
			Direction::Down => self.is_below(r, c),
		}
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
