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

use std::env::current_exe;
use piston_window::*;
use gfx_graphics::GlyphCache;

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
		
		
	
    // Create a new game and run it.
    let mut app = App::new(SIZE[0],SIZE[1]);
	
	let mut frames = 0;
	let mut passed = 0.0;
	
	let exe_directory = current_exe().unwrap().parent().unwrap().to_owned();
	let path = &exe_directory.join("resources/FiraMono-Bold.ttf");
	
	println!("path: {}", path.to_str().unwrap());
	
	let mut font = GlyphCache::new(path,window.factory.clone()).unwrap();

    while let Some(e) = window.next() {
			
		if let Some(_) = e.render_args() {
			window.draw_2d(&e,|c, g| {
				app.render(&c,g);
				frames+=1;
				
				// Render the score
				let mut text = Text::new(22);
				text.color = [0.0, 0.0, 1.0, 1.0];
				text.draw(&format!("Score: {}", 5.0),
						  &mut font,
						  &c.draw_state,
						  c.trans(10.0, 20.0).transform,
						  g);
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