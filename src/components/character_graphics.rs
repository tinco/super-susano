use graphics::{Image, types, ImageSize};
use graphics::math::Scalar;
use opengl_graphics::{Texture};

pub struct CharacterGraphics {
	pub idle_animation: AnimatedSprite
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
			self.frame = (self.frame + 1) % self.textures.len();
			self.start_time = self.start_time - 0.1667;
		}
	}

	pub fn draw() {
	}
}