use graphics::{Image, types, ImageSize};
use graphics::math::Scalar;
use opengl_graphics::{Texture};

#[derive(Copy, Clone)]
pub enum AnimationIndex {
	Idle = 0,
	Punch = 1
}

pub struct CharacterGraphics {
	pub animations: Vec<AnimatedSprite>,
	pub active_animation_index: AnimationIndex
}

impl CharacterGraphics {
	pub fn new(animations: Vec<AnimatedSprite>) -> CharacterGraphics {
		return CharacterGraphics {
			animations: animations,
			active_animation_index: AnimationIndex::Idle
		};
	}

	pub fn update(&mut self, dt: f64) {
		let mut start_idle = false;
		{
			let animation = self.active_animation_mut();
			animation.update(dt);
			if animation.frame >= animation.textures.len() {
				start_idle = true;
			}
		}

		if start_idle {
			self.start_animation(AnimationIndex::Idle);
		}
	}

	pub fn start_animation(&mut self, index: AnimationIndex) {
		self.active_animation_index = index;
		self.active_animation_mut().start();
	}

	pub fn active_animation(&self) -> &AnimatedSprite {
		let index = self.active_animation_index as usize;
		return &self.animations[index];
	}

	pub fn active_animation_mut(&mut self) -> &mut AnimatedSprite {
		let index = self.active_animation_index as usize;
		return &mut self.animations[index];
	}
}

pub struct AnimatedSprite {
	pub textures: Vec<Texture>,
	pub frame: usize,
	pub start_time: f64,
	pub width: f64,
	pub height: f64,
	pub image: Image
}

fn rect(x: Scalar,y: Scalar,h: Scalar,w: Scalar) -> types::Rectangle {
	[x,y, h, w]
}

impl AnimatedSprite {
	pub fn new(textures: Vec<Texture>) -> AnimatedSprite {
		let (width, height) = textures[0].get_size();
		let image = Image::new().rect(rect(0.0,0.0, width as f64, height as f64));

		return AnimatedSprite {
			textures: textures,
			frame: 0,
			start_time: 0.0,
			width: width as f64,
			height: height as f64,
			image: image
		};
	}

	pub fn update(&mut self, dt: f64) {
		self.start_time = self.start_time + dt;

		if self.start_time >= 0.1667 {
			self.frame = self.frame + 1;
			self.start_time = self.start_time - 0.1667;
		}
	}

	pub fn start(&mut self) {
		self.start_time = 0.0;
		self.frame = 0;
	}

	pub fn draw() {
	}
}
