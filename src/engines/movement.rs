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

		{
			let ref mut controlled = entities[controlled_id];

			let collided_entity = if let Some (boundary) = controlled.physical_boundary {
					find_colliding_entity(controlled.position, boundary, entities)
			} else { None };

			if let Some (collided_entity) = collided_entity {
				controlled.position = original_position;
			}
		}

		{
			let ref mut controlled = entities[controlled_id];
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

	fn find_colliding_entity<'a>(position: [f64; 2], boundary: Boundary, entities: &'a Vec<Entity>) -> Option<&'a Entity> {
		for e in entities {
			if let Some(other_boundary) = e.physical_boundary {
				if boundaries_overlap(position, boundary, e.position, other_boundary) {
					return Some(e);
				}
			}
		}
		return None;
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
