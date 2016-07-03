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
extern crate gfx_core;
extern crate gfx_texture;
extern crate gfx_device_gl;
extern crate opengl_graphics;


use std::env::current_exe;
use piston_window::*;
use gfx_graphics::GlyphCache;
use gfx_device_gl::Factory;
use gfx_device_gl::Resources;

use app::*;


fn main() {
	const SIZE: [u32; 2] = [1024,768];

    // Create an Glutin window.
    let mut window: PistonWindow = WindowSettings::new(
            "lgidcf",
            SIZE
        )
        .exit_on_esc(true)
        .build()
        .unwrap();
		
	
	//create a font
	let exe_directory = current_exe().unwrap().parent().unwrap().to_owned();
	let path = &exe_directory.join("resources/FiraMono-Bold.ttf");
	
	let mut font:GlyphCache<Resources,Factory> = GlyphCache::new(path,window.factory.clone()).unwrap();
	
	//TODO: figure out how to pass this to app...
	
	

	
    // Create a new game and run it.
    let mut app = App::new(SIZE[0],SIZE[1]);
	
	let mut frames = 0;
	let mut passed = 0.0;
	
	
	
    while let Some(e) = window.next() {
			
		if let Some(_) = e.render_args() {
			window.draw_2d(&e,|c, g| {
				app.render(&c,g,&mut font);
				frames+=1;
			});
        }

        if let Some(u) = e.update_args() {
		
			let num_objs = app.num_objects();
		
			app.update(&u);
			
			passed += u.dt;
			
			if passed > 1.0 {
		
				let fps = (frames as f64) / passed;
				
				println!("FPS: {}, {}",fps,num_objs);
				
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