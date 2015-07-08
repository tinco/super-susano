use piston::event::*;
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use opengl_graphics::shader_utils::compile_shader;
use graphics::{Image};
use graphics::rectangle::square;
use gl;

use std::io::{Read};
use std::fs::File;

use image::GenericImage;

use assets::{asset_path};
use components::entity::Entity;

pub struct Graphics {
	gl: GlGraphics, // OpenGL drawing backend.
	animated_sprite: AnimatedSprite,
	image: Image,
	animation_shader: gl::types::GLuint
}

pub struct AnimatedSprite {
	textures: Vec<Texture>,
	frame: usize,
	start_time: f64
}

impl AnimatedSprite {
	pub fn new(textures: Vec<Texture>) -> AnimatedSprite {
		return AnimatedSprite {
			textures: textures,
			frame: 0,
			start_time: 0.0
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

impl Graphics {
	pub fn update(&mut self, args: &UpdateArgs) {
		let animation = &mut self.animated_sprite;
		animation.update(args.dt);
	}

	pub fn render(&mut self, args: &RenderArgs, rectangles:&Vec<Entity>) {
		const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
		
		let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

		let gl = &mut self.gl;
		let image = &self.image;
		let animated_sprite = &self.animated_sprite;
		
		gl.draw(args.viewport(), |c, gl| {
			use graphics::*;

			// Clear the screen.
			clear(GREEN, gl);
			for banaan in rectangles {
				let banaan_transform = c.transform
					.trans(x, y)
					.trans(banaan.position[0], banaan.position[1])
					.rot_rad(banaan.rotation);
				c.transform
					.trans(-25.0,-25.0);
				
				image.draw(&animated_sprite.textures[animated_sprite.frame], default_draw_state(), c.transform, gl);
				rectangle(banaan.color, banaan.shape, banaan_transform, gl);
			}
		});
	}

	pub fn new(opengl: OpenGL) -> Graphics {
		// Create a new game and run it.
		let ryu_idle = vec![
			Texture::from_path(asset_path("bitmaps/ryu/idle-1.png").as_path()).unwrap(),
			Texture::from_path(asset_path("bitmaps/ryu/idle-2.png").as_path()).unwrap(),
			Texture::from_path(asset_path("bitmaps/ryu/idle-3.png").as_path()).unwrap(),
			Texture::from_path(asset_path("bitmaps/ryu/idle-4.png").as_path()).unwrap()
		];
		let image = Image::new().rect(square(0.0, 0.0, 200.0));

		let mut vertex_shader_source = String::new();

		let path = asset_path("shaders/animation/vertex.glsl");
		File::open(path).unwrap().read_to_string(&mut vertex_shader_source).unwrap();

		let animation_shader = compile_shader(gl::VERTEX_SHADER, &vertex_shader_source).unwrap();

		let graphics = Graphics {
			gl: GlGraphics::new(opengl),
			animated_sprite: AnimatedSprite::new(ryu_idle),
			image: image,
			animation_shader: animation_shader
		};

		return graphics;
	}
}
