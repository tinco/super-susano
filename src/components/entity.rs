use components::character_graphics::*;

pub enum MovementType {
	Hadouken,
	PlayerCharacter,
	NPCCharacter,
	NoMovement
}

#[derive(PartialEq, Eq)]
pub enum Direction {
	Left, Right
}

#[derive(Copy, Clone)]
pub enum Boundary {
	Rectangle { height: f64, width: f64 }
}

pub struct Entity {
	pub id: i64,
	pub position: [f64; 2],
	pub rotation: f64,
	pub direction: Direction,
	pub character_graphics: Option<CharacterGraphics>,
	pub physical_boundary: Option<Boundary>,
	pub movement_type: MovementType
}

impl PartialEq for Entity {
	fn eq(&self, other: &Entity) -> bool {
		return self.id == other.id;
	}
}
