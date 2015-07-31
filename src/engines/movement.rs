use piston::input::UpdateArgs;
use components::entity::{Entity,Direction, Boundary};
use components::character_graphics::{AnimationIndex};
use engines::input::Input;

use piston::input::Button::*;
use piston::input::keyboard::Key;

pub struct Movement;

impl Movement {
	pub fn update(
		&mut self,
		args: &UpdateArgs,
		entities: &mut Vec<Entity>,
		inputstate: &Input
	) {
		let controlled_id = 0;
		let original_position = entities[controlled_id].position;
		
		{
			let ref mut controlled = entities[controlled_id];
			let mut walking = false;

			for btn in &inputstate.held_buttons {
				match btn {
					&Keyboard(Key::W) => {
						controlled.position[1] -= 100.0 * args.dt;
						walking = true;
					},
					&Keyboard(Key::S) => {
						controlled.position[1] += 100.0 * args.dt;
						walking = true;
					},
					&Keyboard(Key::A) => {
						controlled.position[0] -= 100.0 * args.dt;
						controlled.direction = Direction::Left;
						walking = true;		
					},
					&Keyboard(Key::D) => {
						controlled.position[0] += 100.0 * args.dt;
						controlled.direction = Direction::Right;
						walking = true;		
					},
					_ => ()
				}
			}

			if let Some(ref mut character_graphics) = controlled.character_graphics {
				if walking && (character_graphics.active_animation_index == AnimationIndex::Idle) {
					character_graphics.start_animation(AnimationIndex::Walking);
				} else if !walking && (character_graphics.active_animation_index == AnimationIndex::Walking) {
					character_graphics.start_animation(AnimationIndex::Idle);
				}
			}
		}


		let mut collided = false;
		{
			let ref controlled = entities[controlled_id];

			for n in (0..entities.len()) {
				if n != controlled_id {
					let ref other_entity = entities[n];
					if overlaps_with(controlled, other_entity) {
						collided = true;
						break;
					}
				}
			}
		}

		if collided {
			let ref mut controlled = entities[controlled_id];
			controlled.position = original_position;
		}

		{
			let ref mut  controlled = entities[controlled_id];
			if let Some(ref mut character_graphics) = controlled.character_graphics {
				if inputstate.is_pressed(Keyboard(Key::F)) {
					character_graphics.start_animation(AnimationIndex::Punch);
				}
			}
		}
	}

	pub fn new() -> Movement {
		// Create a new game and run it.
		let movement = Movement;

		return movement;
	}
}

	fn overlaps_with(entity: &Entity, other_entity: &Entity) -> bool {
		if let Some(boundary) = entity.physical_boundary {
			if let Some(other_boundary) = other_entity.physical_boundary {
				return boundaries_overlap(entity.position, boundary, other_entity.position, other_boundary);
			}
		}
		return false;
	}

	fn lies_within(value: f64, minimum: f64, maximum: f64) -> bool {
		if value < minimum { return false };
		if value > maximum { return false };
		return true;
	}

	fn boundaries_overlap(position: [f64; 2], boundary: Boundary, other_position: [f64; 2], other_boundary: Boundary) -> bool {
		match boundary {
			Boundary::Rectangle { height, width } => {
				match other_boundary {
					Boundary::Rectangle {height: other_height, width: other_width } => {
						let position_left = position[0];
						let position_right = position[0] + width;
						let other_left = other_position[0];
						let other_right = other_position[0] + other_width;
						let position_top = position[1];
						let position_bottom = position[1] + height;
						let other_top = other_position[1];
						let other_bottom = other_position[1] + other_height;

						let overlap_x =  lies_within(position_left, other_left, other_right) ||
		 								lies_within(position_right, other_left, other_right);

		 				let overlap_y = lies_within(position_top, other_top, other_bottom) ||
		 				               lies_within(position_bottom, other_top, other_bottom);

		 				return overlap_x && overlap_y;
					}
				}
			}
		}		
		return false;
	}
