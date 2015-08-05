use piston::input::UpdateArgs;
use components::entity::{Entity,Direction, Boundary, MovementType};
use components::character_graphics::{AnimationIndex};
use engines::input::Input;

use piston::input::Button::*;
use piston::input::keyboard::Key;

use assets::{asset_path};
use opengl_graphics::{OpenGL, Texture};
use components::character_graphics::{CharacterGraphics,AnimatedSprite};

pub struct Movement;

impl Movement {
	pub fn update(
		&mut self,
		args: &UpdateArgs,
		entities: &mut Vec<Entity>,
		inputstate: &Input
	) {
		let controlled_id = 1;
		let original_position = entities[controlled_id].position;
		
		{
			let controlled = &mut entities[controlled_id];
			let mut walking = false;

			for btn in &inputstate.held_buttons {
				match btn {
					&Keyboard(Key::W) => {
						controlled.position[1] -= 1.5;
						walking = true;
					},
					&Keyboard(Key::S) => {
						controlled.position[1] += 1.5;
						walking = true;
					},
					&Keyboard(Key::A) => {
						controlled.position[0] -= 1.5;
						controlled.direction = Direction::Left;
						walking = true;		
					},
					&Keyboard(Key::D) => {
						controlled.position[0] += 1.5;
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

		let mut spawn_hadouken = None;
		{
			let controlled = &mut entities[controlled_id];
			if let Some(ref mut character_graphics) = controlled.character_graphics {
				if inputstate.is_pressed(Keyboard(Key::F)) {
					character_graphics.start_animation(AnimationIndex::Punch);
				}

				if inputstate.is_pressed(Keyboard(Key::T)) {
					spawn_hadouken = Some (controlled.position);
				}
			}
		}

		if let Some (hadouken_position) = spawn_hadouken {
			let hadouken_textures = vec![Texture::from_path(asset_path("bitmaps/ryu/hadouken-ball-1.gif").as_path()).unwrap()];
			let hadouken_ball = Entity {
				id: 4,
				position: [hadouken_position[0] + 100.0,hadouken_position[1] - 10.0],
				rotation: 0.0,
				direction: Direction::Right,
				movement_type: MovementType::Hadouken,
				physical_boundary: Some (Boundary::Rectangle {
					width: 10.0,
					height: 10.0
				}),
				character_graphics: Some (CharacterGraphics::new(
					vec![
						AnimatedSprite::new(hadouken_textures, 0.1667)
					]
				))
			};
			entities.push(hadouken_ball);
		}

		{
			for e in entities.iter_mut() {
				match e.movement_type {
					MovementType::Hadouken => e.position[0] += 1.5,
					_ => ()
				}
			}
		}

		let mut collisions : Vec<[i64;2]> = Vec::new();		
		{
			let immut_entities = &*entities;
			for e in immut_entities {
				for e2 in immut_entities {
					if e != e2 {
						if overlaps_with(e, e2) {
							collisions.push([e.id, e2.id]);
						}
					}
				}
			}
		}

		{
			let mut collision_firsts = collisions.iter().map(|&x| x[0]);
			entities.retain(|e| e.movement_type != MovementType::Hadouken || !collision_firsts.any(|c| c == e.id));
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
