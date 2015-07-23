use piston::event::*;

use components::entity::{Entity,Direction};
use engines::input::Input;

use piston::input::Button::*;
use piston::input::keyboard::Key;

pub struct Movement;

impl Movement {
	pub fn update(
		&mut self, 
		args: &UpdateArgs, 
		rectangles: &mut Vec<Entity>,
		inputstate: &Input
	) {

		// Rotate 2 radians per second.
		for banaan in rectangles.iter_mut() {
			banaan.rotation += 2.0 * args.dt;
		}

		let ref mut controlled = rectangles[0];

		if inputstate.held_buttons.contains(&Keyboard(Key::W)) {
			controlled.position[1] -= 100.0 * args.dt;
		}

		if inputstate.held_buttons.contains(&Keyboard(Key::S)) {
			controlled.position[1] += 100.0 * args.dt;
		}

		if inputstate.held_buttons.contains(&Keyboard(Key::A)) {
			controlled.position[0] -= 100.0 * args.dt;
			controlled.direction = Direction::Left;
		}

		if inputstate.held_buttons.contains(&Keyboard(Key::D)) {
			controlled.position[0] += 100.0 * args.dt;
			controlled.direction = Direction::Right;
		}
	}

	pub fn new() -> Movement {
		// Create a new game and run it.
		let movement = Movement;

		return movement;
	}
}