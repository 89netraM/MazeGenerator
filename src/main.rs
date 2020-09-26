mod map;
use map::WallJunction;

fn main() {
	let mut a = WallJunction::new();
	a.set_up(false);
	println!("T junction: {}", a);
}
