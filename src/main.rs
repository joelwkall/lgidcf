mod appdata;
mod app;
mod projectile;
mod player;
mod device;
mod settings;

extern crate piston_window;
extern crate piston;
extern crate rand;
extern crate rustc_serialize;
extern crate graphics;
extern crate gfx_graphics;
extern crate gfx_device_gl;


use std::env::current_exe;
use piston_window::*;
use gfx_graphics::GlyphCache;
use gfx_device_gl::Factory;
use gfx_device_gl::Resources;

use app::*;


fn main() {
	const SIZE: [u32; 2] = [800,600];

	// Create an Glutin window.
	let mut window: PistonWindow = WindowSettings::new(
			"lgidcf",
			SIZE
		)
		.opengl(OpenGL::V2_1)
		.srgb(false)
		.vsync(true)
		//.fullscreen(true)
		.exit_on_esc(true)
		.build()
		.unwrap();
		
	
	//create a font
	let exe_directory = current_exe().unwrap().parent().unwrap().to_owned();
	let path = &exe_directory.join("resources/FiraMono-Bold.ttf");
	
	let mut font:GlyphCache<Resources,Factory> = GlyphCache::new(path,window.factory.clone()).unwrap();

	
	// Create a new game and run it.
	let mut app = App::new(SIZE[0],SIZE[1]);
	
	let mut frames = 0;
	let mut passed = 0.0;
	
	let mut fps:f64 = 0.0;
	
	
	while let Some(e) = window.next() {
			
		if let Some(_) = e.render_args() {
			window.draw_2d(&e,|c, g| {
				app.render(&c,g,&mut font);
				frames+=1;
				
				//print debug info
				let num_objs = app.num_objects();
				
				let mut text = Text::new(20);
				text.color = [0.0, 0.0, 1.0, 1.0];
				text.draw(&format!("FPS: {}, Objects: {}", fps.round() as i32,num_objs),
				  &mut font,
				  &c.draw_state,
				  c.trans((SIZE[0] as f64)-250.0, 20.0).transform,
				  g); 
			});
		}

		if let Some(u) = e.update_args() {
		
		
			app.update(&u);
			
			passed += u.dt;
			
			if passed > 1.0 {
		
				fps = (frames as f64) / passed;
				
				frames = 0;
				passed = 0.0;
			
			}
		}
		
		if let Some(b) = e.press_args() {
			app.handle_button_pressed(b);
		}
		
		if let Some(b) = e.release_args() {
			app.handle_button_released(b);
		}
		
		if let Some(m) = e.mouse_cursor_args() {
			app.handle_mouse_move(m);
		}
		
		
		
		
		
	}
}