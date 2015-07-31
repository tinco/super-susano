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

use piston::input::*;
use piston_window::{ PistonWindow, WindowSettings, Size };

use components::entity::{Entity,Direction, Boundary};
use components::character_graphics::{CharacterGraphics,AnimatedSprite};
use assets::{asset_path};
use opengl_graphics::{OpenGL, Texture};

fn main() {
	let window_settings = WindowSettings::new(
		"Super Susano".to_string(),
		Size { width: 800, height: 400 }
	).exit_on_esc(true);

	// Create an SDL window.
	let window: PistonWindow = window_settings.exit_on_esc(true).into();

	let stage_background = vec![
		Texture::from_path(asset_path("bitmaps/background-1.jpg").as_path()).unwrap()
	];

	let ryu_idle = vec![
		Texture::from_path(asset_path("bitmaps/ryu/idle-1.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/ryu/idle-2.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/ryu/idle-3.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/ryu/idle-4.png").as_path()).unwrap()
	];

	let ryu_punch = vec![
		Texture::from_path(asset_path("bitmaps/ryu/punch-1.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/ryu/punch-2.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/ryu/punch-3.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/ryu/punch-4.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/ryu/punch-5.png").as_path()).unwrap()
	];

	let ryu_walking = vec![
		Texture::from_path(asset_path("bitmaps/ryu/walking-1.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/ryu/walking-2.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/ryu/walking-3.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/ryu/walking-4.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/ryu/walking-5.png").as_path()).unwrap()
	];

	let chun_idle = vec![
		Texture::from_path(asset_path("bitmaps/chun-li/idle-1.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/chun-li/idle-2.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/chun-li/idle-3.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/chun-li/idle-4.png").as_path()).unwrap()
	];

	let chun_punch = vec![
		Texture::from_path(asset_path("bitmaps/chun-li/punch-1.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/chun-li/punch-2.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/chun-li/punch-3.png").as_path()).unwrap()
	];

	let chun_walking = vec![
		Texture::from_path(asset_path("bitmaps/chun-li/walking-1.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/chun-li/walking-2.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/chun-li/walking-3.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/chun-li/walking-4.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/chun-li/walking-5.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/chun-li/walking-6.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/chun-li/walking-7.png").as_path()).unwrap(),
		Texture::from_path(asset_path("bitmaps/chun-li/walking-8.png").as_path()).unwrap()
	];

	let mut rectangles = vec![
		Entity {
			id: 3,
			position: [-0.0,-130.0],
			rotation: 0.0,
			direction: Direction::Left,
			physical_boundary: None,
			character_graphics: Some (CharacterGraphics::new(
				vec![
					AnimatedSprite::new(stage_background, 0.1667)
				]
			))
		},
		Entity {
			id: 1,
			position: [-200.0,0.0],
			rotation: 0.0,
			direction: Direction::Right,
			character_graphics: Some (CharacterGraphics::new(
				vec![
					AnimatedSprite::new(ryu_idle, 0.1667),
					AnimatedSprite::new(ryu_punch, 0.1),
					AnimatedSprite::new(ryu_walking, 0.1667)
				]
			)),
			physical_boundary: Some (Boundary::Rectangle {
				width: 45.0,
				height: 80.0
			})
		},
		Entity {
			id: 2,
			position: [200.0,0.0],
			rotation: 0.0,
			direction: Direction::Left,
			character_graphics: Some (CharacterGraphics::new(
				vec![
					AnimatedSprite::new(chun_idle, 0.1667),
					AnimatedSprite::new(chun_punch, 0.1),
					AnimatedSprite::new(chun_walking, 0.1667)
				]
			)),
			physical_boundary: Some (Boundary::Rectangle {
				width: 45.0,
				height: 80.0
			})
		}
	];


	let opengl = OpenGL::V3_3;
	let mut graphics_engine = engines::graphics::Graphics::new(opengl);
	let mut movement_engine = engines::movement::Movement::new();
	let mut input_engine = engines::input::Input::new();

	for e in window {
		if let Some(r) = e.render_args() {
			graphics_engine.render(&r, &rectangles);
		}

		if let Some(u) = e.update_args() {
			input_engine.update();
			movement_engine.update(&u, &mut rectangles, &input_engine);
			graphics_engine.update(&u, &mut rectangles);
		}

		if let Some(p) = e.press_args() {
			input_engine.pressed(p);
		}

		if let Some(p) = e.release_args() {
			input_engine.released(p);
		}
	}

}
