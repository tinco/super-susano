use piston::input::{RenderArgs,UpdateArgs};
use components::entity::{Entity,Direction, Boundary};

pub struct Camera {
	pub position: [f64; 2],
	margins: [f64; 4],
	viewport_size: [f64; 2]
}

impl Camera {
	pub fn update(&mut self, args: &UpdateArgs, entities: &mut Vec<Entity>) {
		let controlled_id = 1;
		let player_position = entities.iter().find(|&e| e.id == controlled_id).unwrap().position;

		let right_offset = player_position[0] - (self.position[0] + (self.viewport_size[0] / 2.0) - self.right_margin());
		if right_offset > 0.0 {
			self.position[0] += right_offset;
		}
		let left_offset = (self.position[0] - (self.viewport_size[0] / 2.0) + self.left_margin()) - player_position[0];
		if left_offset > 0.0 {
			self.position[0] -= left_offset;
		}
		
		let bottom_offset = player_position[1] - (self.position[1] + (self.viewport_size[1] / 2.0) - self.bottom_margin());
		if bottom_offset > 0.0 {
			self.position[1] += bottom_offset;
		}
		let top_offset = (self.position[1] - (self.viewport_size[1] / 2.0) + self.top_margin()) - player_position[1];
		if top_offset > 0.0 {
			self.position[1] -= top_offset;
		}
	}

	pub fn render(&mut self, args: &RenderArgs, entities:&Vec<Entity>) {

	}

	pub fn top_margin(&self) -> f64 {
		return self.margins[0];
	}

	pub fn right_margin(&self) -> f64 {
		return self.margins[1];
	}

	pub fn bottom_margin(&self) -> f64 {
		return self.margins[2];
	}

	pub fn left_margin(&self) -> f64 {
		return self.margins[3];
	}

	pub fn new() -> Camera {
		// Create a new game and run it.
		let camera = Camera {
			position: [0.0,0.0],
			viewport_size: [1280.0,720.0],
			margins: [100.0,100.0,100.0,100.0]
		};
		return camera;
	}
}
