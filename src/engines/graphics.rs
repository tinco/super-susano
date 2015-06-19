use piston::event::*;
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use opengl_graphics::shader_utils::compile_shader;
use graphics::{Image,ImageSize,rectangle,clear,default_draw_state};
use graphics::rectangle::square;
use gl;

use std::path::Path;
use std::io::{Read};
use std::fs::File;

use image::GenericImage;
use sprite::*;

use assets::{asset_path};
use components::entity::Entity;

pub struct Graphics<I: ImageSize> {
	gl: GlGraphics, // OpenGL drawing backend.
	ryu: Texture,
	image: Image,
	animation_shader: gl::types::GLuint,
	scene: Scene<I>
}

impl<I: ImageSize> Graphics<I> {
	pub fn render(&mut self, args: &RenderArgs, rectangles:&Vec<Entity>) {
		const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
		
		let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

		let gl = &mut self.gl;
		let image = &self.image;
		let ryu = &self.ryu;
		let scene = &self.scene;
		
		gl.draw(args.viewport(), |c, gl| {
			// Clear the screen.
			//clear(GREEN, gl);

			for banaan in rectangles {
				let banaan_transform = c.transform;

				//	.trans(x, y)
				//	.trans(banaan.position[0], banaan.position[1])
				//	.rot_rad(banaan.rotation);
				//	.trans(-25.0,-25.0);
				
				//image.draw(ryu, default_draw_state(), c.transform, gl);
				//rectangle(banaan.color, banaan.shape, banaan_transform, gl);
			}

			//scene.draw(c.transform, gl);			
		});
	}

	pub fn new(opengl: OpenGL) -> Graphics<I> {
		// Create a new game and run it.
		let path = asset_path("bitmaps/ryu/idle-1.png");
		let ryu = Texture::from_path(path.as_path()).unwrap();
	    let image = Image::new().rect(square(0.0, 0.0, 200.0));

	    let mut scene = Scene::new();
	    //let mut sprite = Sprite::from_texture(ryu);
	    //scene.add_child(sprite);

		let mut vertex_shader_source = String::new();

		let path = asset_path("shaders/animation/vertex.glsl");
		File::open(path).unwrap().read_to_string(&mut vertex_shader_source).unwrap();

	    let animation_shader = compile_shader(gl::VERTEX_SHADER, &vertex_shader_source).unwrap();

		let graphics = Graphics {
			gl: GlGraphics::new(opengl),
			ryu: ryu,
			image: image,
			animation_shader: animation_shader,
			scene: scene
		};

		return graphics;
	}
}
