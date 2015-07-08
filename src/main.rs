extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate image;
extern crate gl;
extern crate sprite;
extern crate texture;
extern crate piston_window;

pub mod engines;
pub mod components;
pub mod assets;

use piston::window::{ WindowSettings, Size };
use piston::event::*;
use piston_window::{ PistonWindow };
use opengl_graphics::OpenGL;
use graphics as PistonGraphics;
use components::entity::Entity;

fn main() {
	let opengl = OpenGL::_3_3;
	let window_settings = WindowSettings::new(
		"Super Susano".to_string(),
		Size { width: 800, height: 400 }
	).exit_on_esc(true);

	// Create an SDL window.
	let window: PistonWindow = window_settings.exit_on_esc(true).into();

	const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
	const BLUE:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];
	const YELLOW:[f32; 4] = [1.0, 1.0, 0.0, 1.0];

	let mut rectangles = vec![
		Entity {
			color: YELLOW,
			shape: PistonGraphics::rectangle::square(0.0, 0.0, 50.0),
			position: [-100.0,0.0],
			rotation: 0.0
		},
		Entity {
			color: BLUE,
			shape: PistonGraphics::rectangle::square(0.0, 0.0, 50.0),
			position: [0.0,0.0],
			rotation: 0.0
		},
		Entity {
			color: RED,
			shape: PistonGraphics::rectangle::square(0.0, 0.0, 50.0),
			position: [100.0,0.0],
			rotation: 0.0
		},
	];


	let mut graphics_engine = engines::graphics::Graphics::new(opengl);
	let mut movement_engine = engines::movement::Movement::new();
	let mut input_engine = engines::input::Input::new();

	for e in window.events() {
		if let Some(r) = e.render_args() {
			graphics_engine.render(&r, &rectangles);
		}

		if let Some(u) = e.update_args() {
			input_engine.update();		
			movement_engine.update(&u, &mut rectangles, &input_engine);
			graphics_engine.update(&u);	
		}

		if let Some(p) = e.press_args() {
			input_engine.pressed(p);
		}

		if let Some(p) = e.release_args() {
			input_engine.released(p);
		}
	}

}