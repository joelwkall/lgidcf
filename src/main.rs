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

	let print_debug = true;

	const SIZE: [u32; 2] = [800,600];

	// Create an Glutin window.
	let mut window: PistonWindow = WindowSettings::new(
			"lgidcf",
			SIZE
		)
		.opengl(OpenGL::V3_3)
		.vsync(true)
		//.fullscreen(true)
		.exit_on_esc(true)
		.build()
		.unwrap();
		
	if print_debug {
		println!("Created window.");
    }
	
	//create a font
	let exe_directory = current_exe().unwrap().parent().unwrap().to_owned();
	let path = &exe_directory.join("resources/FiraMono-Bold.ttf");
	
	let mut font:GlyphCache<Resources,Factory> = GlyphCache::new(path,window.factory.clone()).unwrap();

	if print_debug {
		println!("Instantiated fonts.");
    }
	
	// Create a new game and run it.
	let mut app = App::new(SIZE[0],SIZE[1]);
	
	let mut frames = 0;
	let mut passed = 0.0;
	
	let mut fps:f64 = 0.0;

	if print_debug {
		println!("Created app.");
    }

	
	while let Some(e) = window.next() {
			
		if print_debug {
			println!("Aquired window event.");
		}

		if let Some(_) = e.render_args() {

			if print_debug {
				println!("Beginning render event handling.");
			}

			window.draw_2d(&e,|c, g| {

				if print_debug {
					println!("Beginning draw closure.");
				}

				app.render(&c,g,&mut font);

				if print_debug {
					println!("Finished app drawing.");
				}

				frames+=1;
				
				//print debug info
				let num_objs = app.num_objects();
				
				let mut text = Text::new(10);
				text.color = [0.0, 0.0, 1.0, 1.0];
				text.draw(&format!("FPS: {}, Objects: {}", fps.round() as i32,num_objs),
				  &mut font,
				  &c.draw_state,
				  c.trans((SIZE[0] as f64)-250.0, 20.0).transform,
				  g); 

				if print_debug {
					println!("Finished draw closure.");
				}
			});

			if print_debug {
				println!("Finished render event handling.");
			}

		}

		if let Some(u) = e.update_args() {
		
			if print_debug {
				println!("Beginning update event handling.");
			}
		
			app.update(&u);

			if print_debug {
				println!("Finished app update event handling.");
			}
			
			passed += u.dt;
			
			if passed > 1.0 {
		
				fps = (frames as f64) / passed;
				
				frames = 0;
				passed = 0.0;
			
			}

			if print_debug {
				println!("Finished update event handling.");
			}
		}
		
		if let Some(b) = e.press_args() {

			if print_debug {
				println!("Button pressed.");
			}

			app.handle_button_pressed(b);
		}
		
		if let Some(b) = e.release_args() {

			if print_debug {
				println!("Button released.");
			}

			app.handle_button_released(b);
		}
		
		if let Some(m) = e.mouse_cursor_args() {

			if print_debug {
				println!("Mouse moved.");
			}

			app.handle_mouse_move(m);
		}
		
		
		if print_debug {
			println!("Finished window event. Frames={}",frames);
		}
		
		
	}
}