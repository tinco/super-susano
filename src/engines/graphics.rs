use piston::input::{RenderArgs,UpdateArgs};
use opengl_graphics::{GlGraphics, OpenGL};
use image::GenericImage;
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

	pub fn render(&mut self, args: &RenderArgs, entities:&Vec<Entity>) {
		const GREEN: [f32; 4] = [11.0/256.0, 178.0/256.0, 12.0/256.0, 1.0];
		
		let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

		let gl = &mut self.gl;

		gl.draw(args.viewport(), |c, gl| {
			use graphics::*;

			// Clear the screen.
			clear(GREEN, gl);
			for entity in entities {
				
				if let Some(ref character_graphics) = entity.character_graphics {
					let ref animation = character_graphics.active_animation();
					let half_width = animation.width / 2.0;
					let flipped = entity.direction == Direction::Left;
					let transform = c.transform
						.trans(if flipped { x + half_width } else { x - half_width }, y)
						.trans(entity.position[0], entity.position[1])
						.scale(if flipped { - 1.0 } else { 1.0 },1.0);
					animation.image.draw(&animation.textures[animation.frame], default_draw_state(), transform, gl);
				} else {
					let rotating_transform = c.transform
						.trans(x, y)
						.trans(entity.position[0], entity.position[1])
						.rot_rad(entity.rotation);
					c.transform
						.trans(-25.0,-25.0);	
					rectangle(entity.color, entity.shape, rotating_transform, gl);
				}
			}
		});
	}

	pub fn new(opengl: OpenGL) -> Graphics {
		let graphics = Graphics {
			gl: GlGraphics::new(opengl)
		};

		return graphics;
	}
}
