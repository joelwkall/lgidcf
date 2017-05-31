mod appdata;
mod app;
mod projectile;
mod player;
mod device;
mod settings;
mod obstacle;

extern crate piston_window;
extern crate piston;
extern crate rand;
extern crate rustc_serialize;
extern crate graphics;
extern crate gfx_graphics;
extern crate gfx_device_gl;
extern crate chrono;
extern crate time;

use std::env::current_exe;
use piston_window::*;
use gfx_graphics::GlyphCache;
use gfx_device_gl::Factory;
use gfx_device_gl::Resources;

use app::*;


fn main() {


    const MAP_SIZE: [f64; 2] = [1600.0,1200.0];
	const WINDOW_SIZE: [f64; 2] = [800.0,600.0];

	// Create an Glutin window.
	let mut window: PistonWindow = WindowSettings::new(
			"lgidcf",
			[WINDOW_SIZE[0] as u32,WINDOW_SIZE[1] as u32]
		)
		.opengl(OpenGL::V3_3)
		.vsync(true)
		//.fullscreen(true)
		.exit_on_esc(true)
		.build()
		.unwrap();
		
	if cfg!(debug_assertions) {
		println!("Created window.");
    }
	
	//create a font
	let exe_directory = current_exe().unwrap().parent().unwrap().to_owned();
	let path = &exe_directory.join("resources/FiraMono-Bold.ttf");
	
	let mut font:GlyphCache<Resources,Factory> = GlyphCache::new(path,window.factory.clone()).unwrap();

	if cfg!(debug_assertions) {
		println!("Instantiated fonts.");
    }
	
	// Create a new game and run it.
	let mut app = App::new(MAP_SIZE,WINDOW_SIZE);
	
	let mut frames = 0;
	let mut prev_time = chrono::UTC::now();
	
	let mut fps:f64 = 0.0;

	if cfg!(debug_assertions) {
		println!("Created app.");
    }

	while let Some(e) = window.next() {
			
		if cfg!(debug_assertions) {
			//println!("Aquired window event.");
		}

		if let Some(_) = e.render_args() {

			if cfg!(debug_assertions) {
				//println!("Beginning render event handling.");
			}

			window.draw_2d(&e,|c, g| {

				if cfg!(debug_assertions) {
					//println!("Beginning draw closure.");
				}

				app.render(&c,g,&mut font);

				if cfg!(debug_assertions) {
					//println!("Finished app drawing.");
				}

				frames+=1;
				
				//print debug info
				let num_objs = app.num_objects();
				
				let mut text = Text::new(10);
				text.color = [0.0, 0.0, 1.0, 1.0];
				text.draw(&format!("FPS: {}, Objects: {}", fps.round() as i32,num_objs),
				  &mut font,
				  &c.draw_state,
				  c.trans(WINDOW_SIZE[0]-250.0, 20.0).transform,
				  g); 

				if cfg!(debug_assertions) {
					//println!("Finished draw closure.");
				}
			});

			if cfg!(debug_assertions) {
				//println!("Finished render event handling.");
			}

		}

		if let Some(u) = e.update_args() {
		
			if cfg!(debug_assertions) {
				//println!("Beginning update event handling.");
			}
		
			app.update(&u);

			if cfg!(debug_assertions) {
				//println!("Finished app update event handling.");
			}
			
            //FPS calc
            let seconds = (chrono::UTC::now().signed_duration_since(prev_time).num_milliseconds() as f64)/1000.0;
			
			if seconds > 2.0 {
				fps = (frames as f64) / seconds;
				
				frames = 0;
			    prev_time = chrono::UTC::now();
			}

			if cfg!(debug_assertions) {
				//println!("Finished update event handling.");
			}
		}
		
		if let Some(b) = e.press_args() {

			if cfg!(debug_assertions) {
				//println!("Button pressed.");
			}

			app.handle_button_pressed(b);
		}
		
		if let Some(b) = e.release_args() {

			if cfg!(debug_assertions) {
				//println!("Button released.");
			}

			app.handle_button_released(b);
		}
		
		if let Some(m) = e.mouse_cursor_args() {

			if cfg!(debug_assertions) {
				//println!("Mouse moved.");
			}

			app.handle_mouse_move(m);
		}
		
		
		if cfg!(debug_assertions) {
			//println!("Finished window event. Frames={}",frames);
		}
		
		
	}
}