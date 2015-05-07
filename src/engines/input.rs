use components::entity::Entity;
use piston::input;

pub struct Input {
	pub pressed_buttons: Vec<input::Button>,
	pub released_buttons: Vec<input::Button>,
	pub held_buttons: Vec<input::Button>
}

impl Input {
	pub fn update(&mut self) {
		let pressed_buttons = &mut self.pressed_buttons;
		let released_buttons = &mut self.released_buttons;
		let held_buttons = &mut self.held_buttons;

		for &btn in pressed_buttons.iter() {
			held_buttons.push(btn.clone());
		}

		held_buttons.retain(|btn| !released_buttons.contains(btn));
		pressed_buttons.clear();
		released_buttons.clear();
	}

	pub fn pressed(&mut self, b: input::Button) {
		self.pressed_buttons.push(b);
	}

	pub fn released(&mut self, b: input::Button) {
		self.released_buttons.push(b);
	}

	pub fn new() -> Input {
		let input = Input {
			pressed_buttons: vec![],
			released_buttons: vec![],
			held_buttons: vec![]
		};
		return input;
	}
}