use std::fmt;

const UP: usize = 0b1000;
const LEFT: usize = 0b0100;
const RIGHT: usize = 0b0010;
const DOWN: usize = 0b0001;

pub struct WallJunction(usize);

impl WallJunction {
	pub fn new() -> WallJunction {
		WallJunction(UP | LEFT | RIGHT | DOWN)
	}
	pub fn new_empty() -> WallJunction {
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
