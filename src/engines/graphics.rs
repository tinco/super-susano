use piston::event::*;
use opengl_graphics::{ GlGraphics, OpenGL };

use components::banaan_rectangle::BanaanRectangle;

pub struct Graphics {
	gl: GlGraphics // OpenGL drawing backend.
}

impl Graphics {
	pub fn render(&mut self, args: &RenderArgs, rectangles:&Vec<BanaanRectangle>) {
		use graphics::*;

		const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
		
		let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
		
		self.gl.draw(args.viewport(), |c, gl| {
			// Clear the screen.
			clear(GREEN, gl);

			for banaan in rectangles {
				let banaan_transform = c.transform
					.trans(x, y)
					.trans(banaan.position[0], banaan.position[1])
					.rot_rad(banaan.rotation)
					.trans(-25.0,-25.0);

				rectangle(banaan.color, banaan.shape, banaan_transform, gl);
			}
			
		});
	}

	pub fn new(opengl: OpenGL) -> Graphics {
		// Create a new game and run it.
		let graphics = Graphics {
			gl: GlGraphics::new(opengl)
		};

		return graphics;
	}
}