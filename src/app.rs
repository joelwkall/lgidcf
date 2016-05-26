extern crate piston_window;
extern crate piston;
extern crate rustc_serialize;


use rustc_serialize::json::decode;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;


use piston_window::*;

use appdata::AppData;
use player::Player;
use projectile::Projectile;
use device::*;



pub struct App {
	pub player: Player,
	
	pub data: AppData
}

impl App {

	pub fn new(width:u32,height:u32) -> App {
	
		//device
		let mut data = String::new();
		let mut f = File::open("settings/device.json").unwrap();
		f.read_to_string(&mut data).unwrap();
		let d: Device = decode(&data).unwrap();
		
		//jetpack
		data = String::new();
		f = File::open("settings/jetpack.json").unwrap();
		f.read_to_string(&mut data).unwrap();
		let j: ProjectileTemplate = decode(&data).unwrap();
	
		App {
			player: Player {
				x:(width as f64)/2.0,
				y:(height as f64)/2.0,
				speed_x:0.0,
				speed_y:0.0,
				time_since_shot:0.0,
				device: d,
				jetpack: Rc::new(j)
			},
			data: AppData::new(width,height)
		}
		
		
	}
	
	pub fn num_objects(&self) -> u32 {
	
		return (self.data.projectiles.len() as u32) + 2; //player + mouse cursor
		
	}

    pub fn render(&self, c:&Context, g: &mut G2d) {

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        
		//clear the screen.
		clear(GREEN, g);
		
		for d in &self.data.projectiles {
			d.render(&c,g);
		}
		
		self.player.render(&c,g);
    
    }

    pub fn update(&mut self, args: &UpdateArgs) {
		
	
		let mut new_projectiles:Vec<Projectile> = Vec::new();
		
		for d in &self.data.projectiles {
			
			for p in d.update(&args,&self.data) {
				new_projectiles.push(p);
			}
			
		}

	
		
		match self.player.update(&args,&mut self.data) {
			Some(v) => {
				for p in v {
					new_projectiles.push(p);
				}
			},
			None => {}
		}
		
		//self.data.objects = newObjects;
		self.data.projectiles = new_projectiles;
    }
	
	pub fn handle_button_pressed(&mut self, button: Button) {

		self.data.handle_button_pressed(button);

	}
	
	pub fn handle_button_released(&mut self, button: Button) {

		self.data.handle_button_released(button);

	}
	
	pub fn handle_mouse_move(&mut self, position: [f64;2]) {
		self.data.mouse_x = position[0] as u32;
		self.data.mouse_y = position[1] as u32;
	}

}

