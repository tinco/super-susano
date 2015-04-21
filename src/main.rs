extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

pub mod engines;

fn main() {
	engines::graphics::Graphics::new();
}