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

		let keyW = Keyboard(Key::W);
		if (inputstate.pressed_buttons.contains(&keyW)) {
			rectangles[0].position[0] += 10.0;
		}
	}

	pub fn new() -> Movement {
		// Create a new game and run it.
		let movement = Movement;

		return movement;
	}
}