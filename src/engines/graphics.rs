use piston::window::{ WindowSettings, Size };
use piston::event::*;
use sdl2_window::Sdl2Window as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::{ rectangle, clear };

pub struct Graphics {
	gl: GlGraphics, // OpenGL drawing backend.
	rotation: f64   // Rotation for the square.
}

impl Graphics {
	fn render(&mut self, args: &RenderArgs) {
		use graphics::*;

		const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
		const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

		let square = rectangle::square(0.0, 0.0, 50.0);
		let rotation = self.rotation;
		let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

		self.gl.draw(args.viewport(), |c, gl| {
			// Clear the screen.
			clear(GREEN, gl);

			let transform = c.transform.trans(x, y)
									   .rot_rad(rotation)
									   .trans(-25.0, -25.0);

			// Draw a box rotating around the middle of the screen.
			rectangle(RED, square, transform, gl);
		});
	}

	fn update(&mut self, args: &UpdateArgs) {
		// Rotate 2 radians per second.
		self.rotation += 2.0 * args.dt;
	}

	fn new() {
		let opengl = OpenGL::_3_1;
		let window_settings = WindowSettings::new(
			"Super Susano".to_string(),
			Size { width: 800, height: 400 }
		).exit_on_esc(true);

		// Create an SDL window.
		let window = Window::new(opengl, window_settings);

		// Create a new game and run it.
		let mut graphics = Graphics {
			gl: GlGraphics::new(opengl),
			rotation: 0.0
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