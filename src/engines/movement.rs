use piston::event::*;

use components::entity::Entity;
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

		if (inputstate.held_buttons.contains(&Keyboard(Key::W))) {
			rectangles[0].position[1] -= 100.0 * args.dt;
		}

		if (inputstate.held_buttons.contains(&Keyboard(Key::S))) {
			rectangles[0].position[1] += 100.0 * args.dt;
		}

		if (inputstate.held_buttons.contains(&Keyboard(Key::A))) {
			rectangles[0].position[0] -= 100.0 * args.dt;
		}

		if (inputstate.held_buttons.contains(&Keyboard(Key::D))) {
			rectangles[0].position[0] += 100.0 * args.dt;
		}
	}

	pub fn new() -> Movement {
		// Create a new game and run it.
		let movement = Movement;

		return movement;
	}
}