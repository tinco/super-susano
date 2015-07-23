use piston::event::*;
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use opengl_graphics::shader_utils::compile_shader;
use graphics::{Image, types, ImageSize};
use gl;
use graphics::math::Scalar;

use std::io::{Read};
use std::fs::File;

use image::GenericImage;

use assets::{asset_path};
use components::entity::{Entity,Direction};
use components::character_graphics::{CharacterGraphics};

pub struct Graphics {
	gl: GlGraphics, // OpenGL drawing backend.
	animation_shader: gl::types::GLuint
}

impl Graphics {
	pub fn update(&mut self, args: &UpdateArgs, entities:&mut Vec<Entity>) {
		for entity in entities.iter_mut() {
			if let Some(ref mut character_graphics) = entity.character_graphics {
				let animation = &mut character_graphics.idle_animation;
				animation.update(args.dt);
			}
		}
	}

	pub fn render(&mut self, args: &RenderArgs, rectangles:&Vec<Entity>) {
		const GREEN: [f32; 4] = [11.0/256.0, 178.0/256.0, 12.0/256.0, 1.0];
		
		let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

		let gl = &mut self.gl;

		gl.draw(args.viewport(), |c, gl| {
			use graphics::*;

			// Clear the screen.
			clear(GREEN, gl);
			for banaan in rectangles {
				
				if let Some(ref character_graphics) = banaan.character_graphics {
					let ref animation = character_graphics.idle_animation;
					let transform = c.transform
						.trans(x, y)
						.trans(banaan.position[0], banaan.position[1])
						.scale(if banaan.direction == Direction::Right { 1.0 } else { -1.0 },1.0);
					animation.image.draw(&animation.textures[animation.frame], default_draw_state(), transform, gl);
				} else {
					let banaan_transform = c.transform
						.trans(x, y)
						.trans(banaan.position[0], banaan.position[1])
						.rot_rad(banaan.rotation);
					c.transform
						.trans(-25.0,-25.0);	
					rectangle(banaan.color, banaan.shape, banaan_transform, gl);
				}
			}
		});
	}

	pub fn new(opengl: OpenGL) -> Graphics {
		let mut vertex_shader_source = String::new();

		let path = asset_path("shaders/animation/vertex.glsl");
		File::open(path).unwrap().read_to_string(&mut vertex_shader_source).unwrap();

		let animation_shader = compile_shader(gl::VERTEX_SHADER, &vertex_shader_source).unwrap();

		let graphics = Graphics {
			gl: GlGraphics::new(opengl),
			animation_shader: animation_shader
		};

		return graphics;
	}
}
