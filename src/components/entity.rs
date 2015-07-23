use components::character_graphics::*;

#[derive(PartialEq, Eq)]
pub enum Direction {
	Left, Right
}

pub struct Entity {
	pub color: [f32; 4],
	pub shape: [f64; 4],
	pub position: [f64; 2],
	pub rotation: f64,
	pub direction: Direction,
	pub character_graphics: Option<CharacterGraphics>
}
