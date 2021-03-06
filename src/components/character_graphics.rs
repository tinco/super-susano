use graphics::{Image, types, ImageSize};
use graphics::math::Scalar;
use opengl_graphics::{Texture};

#[derive(Copy, Clone,PartialEq,Eq)]
pub enum AnimationIndex {
	Idle = 0,
	Punch = 1,
	Walking = 2
}

pub struct CharacterGraphics {
	pub width: f64,
	pub animations: Vec<AnimatedSprite>,
	pub active_animation_index: AnimationIndex
}

impl CharacterGraphics {
	pub fn new(animations: Vec<AnimatedSprite>) -> CharacterGraphics {
		return CharacterGraphics {
			width: animations[0].width,
			animations: animations,
			active_animation_index: AnimationIndex::Idle
		};
	}

	pub fn update(&mut self) {
		let start_idle;
		{
			let animation = self.active_animation_mut();
			animation.update();
			start_idle = animation.done;
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
	pub speed: f64,
	pub reverse: bool,
	pub done: bool,
	pub image: Image
}

fn rect(x: Scalar,y: Scalar,h: Scalar,w: Scalar) -> types::Rectangle {
	[x,y, h, w]
}

impl AnimatedSprite {
	pub fn new(textures: Vec<Texture>, speed: f64) -> AnimatedSprite {
		let (width, height) = textures[0].get_size();
		let image = Image::new().rect(rect(0.0,0.0, width as f64, height as f64));

		return AnimatedSprite {
			textures: textures,
			frame: 0,
			start_time: 0.0,
			width: width as f64,
			height: height as f64,
			speed: speed,
			reverse: false,
			done: false,
			image: image
		};
	}

	pub fn update(&mut self) {
		self.start_time = self.start_time + (1.0 / 60.0);

		if self.start_time >= self.speed {
			self.frame = self.frame + 1;
			self.start_time = self.start_time - self.speed;
		}

		if self.frame >= self.textures.len() {
			self.done = true;
		}
	}

	pub fn start(&mut self) {
		self.start_time = 0.0;
		self.frame = 0;
		self.done = false;
	}

	pub fn draw() {
	}
}
