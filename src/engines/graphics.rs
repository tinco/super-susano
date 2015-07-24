use piston::event::*;
use opengl_graphics::{GlGraphics, OpenGL};

use std::io::{Read};
use std::fs::File;

use image::GenericImage;

use assets::{asset_path};
use components::entity::{Entity,Direction};

pub struct Graphics {
	gl: GlGraphics // OpenGL drawing backend.
}

impl Graphics {
	pub fn update(&mut self, args: &UpdateArgs, entities:&mut Vec<Entity>) {
		for entity in entities.iter_mut() {
			if let Some(ref mut character_graphics) = entity.character_graphics {
				let animation = character_graphics.active_animation_mut();
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
					let ref animation = character_graphics.active_animation();
					let half_width = animation.width / 2.0;
					let flipped = banaan.direction == Direction::Left;
					let transform = c.transform
						.trans(if flipped { x + half_width } else { x - half_width }, y)
						.trans(banaan.position[0], banaan.position[1])
						.scale(if flipped { - 1.0 } else { 1.0 },1.0);
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

		let graphics = Graphics {
			gl: GlGraphics::new(opengl)
		};

		return graphics;
	}
}
