use piston::window::{ WindowSettings, Size };
use piston::event::*;
use sdl2_window::Sdl2Window as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics as PistonGraphics;

struct BanaanRectangle {
	color: [f32; 4],
	shape: [f64; 4],
	position: [f64; 2],
	rotation: f64
}

pub struct Graphics {
	gl: GlGraphics, // OpenGL drawing backend.
	rectangles: Vec<BanaanRectangle>
}

impl Graphics {
	fn render(&mut self, args: &RenderArgs) {
		use graphics::*;

		const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
		
		let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
		let rectangles = &self.rectangles;
		
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

	fn update(&mut self, args: &UpdateArgs) {
		// Rotate 2 radians per second.
		for banaan in &mut self.rectangles {
			banaan.rotation += 2.0 * args.dt;
		}
	}

	pub fn new() {
		let opengl = OpenGL::_3_1;
		let window_settings = WindowSettings::new(
			"Super Susano".to_string(),
			Size { width: 800, height: 400 }
		).exit_on_esc(true);

		// Create an SDL window.
		let window = Window::new(opengl, window_settings);

		const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
		const BLUE:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];
		const YELLOW:[f32; 4] = [1.0, 1.0, 0.0, 1.0];

		// Create a new game and run it.
		let mut graphics = Graphics {
			gl: GlGraphics::new(opengl),
			rectangles: vec![
				BanaanRectangle {
					color: YELLOW,
					shape: PistonGraphics::rectangle::square(0.0, 0.0, 50.0),
					position: [-100.0,0.0],
					rotation: 0.0
				},
				BanaanRectangle {
					color: BLUE,
					shape: PistonGraphics::rectangle::square(0.0, 0.0, 50.0),
					position: [0.0,0.0],
					rotation: 0.0
				},
				BanaanRectangle {
					color: RED,
					shape: PistonGraphics::rectangle::square(0.0, 0.0, 50.0),
					position: [100.0,0.0],
					rotation: 0.0
				},
			]
		};

		for e in window.events() {
			if let Some(r) = e.render_args() {
				graphics.render(&r);
			}

			if let Some(u) = e.update_args() {
				graphics.update(&u);
			}
		}

	}
}