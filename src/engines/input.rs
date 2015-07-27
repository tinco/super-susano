use piston::input;
use std::mem::swap;

pub struct Input {
	pub unprocessed_pressed_buttons: Vec<input::Button>,
	pub pressed_buttons: Vec<input::Button>,
	pub released_buttons: Vec<input::Button>,
	pub held_buttons: Vec<input::Button>
}

impl Input {
	pub fn update(&mut self) {
		let unprocessed_pressed_buttons = &mut self.unprocessed_pressed_buttons;
		let pressed_buttons = &mut self.pressed_buttons;
		let released_buttons = &mut self.released_buttons;
		let held_buttons = &mut self.held_buttons;

		swap(unprocessed_pressed_buttons, pressed_buttons);
		unprocessed_pressed_buttons.clear();

		for &btn in pressed_buttons.iter() {
			held_buttons.push(btn.clone());
		}

		held_buttons.retain(|btn| !released_buttons.contains(btn));
		released_buttons.clear();
	}

	pub fn pressed(&mut self, b: input::Button) {
		self.unprocessed_pressed_buttons.push(b);
	}

	pub fn released(&mut self, b: input::Button) {
		self.released_buttons.push(b);
	}

	pub fn new() -> Input {
		let input = Input {
			unprocessed_pressed_buttons: vec![],
			pressed_buttons: vec![],
			released_buttons: vec![],
			held_buttons: vec![]
		};
		return input;
	}
}
