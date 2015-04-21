use piston::event::*;

use components::banaan_rectangle::BanaanRectangle;

pub struct Movement;

impl Movement {
	pub fn update(&mut self, args: &UpdateArgs, rectangles: &mut Vec<BanaanRectangle>) {
		// Rotate 2 radians per second.
		for banaan in rectangles {
			banaan.rotation += 2.0 * args.dt;
		}
	}

	pub fn new() -> Movement {
		// Create a new game and run it.
		let movement = Movement;

		return movement;
	}
}