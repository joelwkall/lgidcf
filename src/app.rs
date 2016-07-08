


use rustc_serialize::json::decode;
use std::fs::File;
use std::fs;
use std::io::Read;
use std::rc::Rc;

use piston_window::*;


use appdata::AppData;
use player::Player;
use projectile::Projectile;
use device::*;
use settings::*;
use gfx_graphics::GlyphCache;
use gfx_device_gl::Factory;
use gfx_device_gl::Resources;


pub struct App {
	pub players: Vec<Player>,
	
	pub data: AppData
}



impl App {

	pub fn new(width:u32,height:u32) -> App {
	
		
	
		//devices
		let paths = fs::read_dir("settings/devices").unwrap();
		let mut devices = Vec::new();
		for path in paths {
			let mut data = String::new();
			let mut f = File::open(path.unwrap().path()).unwrap();
			f.read_to_string(&mut data).unwrap();
			let d: Device = decode(&data).unwrap();
			devices.push(d);
		}
		
		
		let app_data = AppData::new(width,height,devices);
		
		//jetpack
		let mut data = String::new();
		let mut f = File::open("settings/jetpack.json").unwrap();
		f.read_to_string(&mut data).unwrap();
		let j: Rc<ProjectileTemplate> = decode(&data).unwrap();
		
		//settings
		data = String::new();
		f = File::open("settings/settings.json").unwrap();
		f.read_to_string(&mut data).unwrap();
		let s: Settings = decode(&data).unwrap();
		
		let mut players = Vec::new();
		
		let space = (width as f64) / (s.players.len() as f64+1.0);
		
		for (i,p) in s.players.iter().enumerate() {
		
			let player = Player {
				x:space*(i as f64 + 1.0),
				y:(height as f64)/2.0,
				speed_x:0.0,
				speed_y:0.0,
				time_since_shot:0.0,
				dir:[0.0,0.0],
				jetpack: j.clone(),
				settings: p.clone(),
				health:100.0,
				index:i as i32,
				name: "Noname".to_string(),
				current_device:0,
				switch_was_pressed:false
			};
			
			players.push(player);
		};
	
		App {
			players: players,
			data: app_data
		}
		
		
	}
	
	pub fn num_objects(&self) -> u32 {
	
		return (self.data.projectiles.len() as u32) + 2; //player + mouse cursor
		
	}

	pub fn render(&self, c:&Context, g: &mut G2d,font: &mut GlyphCache<Resources,Factory>) {

		const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
		
		//clear the screen.
		clear(GREEN, g);
		
		for d in &self.data.projectiles {
			d.render(&c,g);
		}
		
		
		
		for p in &self.players {
			p.render(&c,g,&self.data,font);
			
			
		}
		
		
		
	
	}

	pub fn update(&mut self, args: &UpdateArgs) {
		
	
		let mut new_projectiles:Vec<Projectile> = Vec::new();
		
		for p in &mut self.players {
			match p.update(&args,&mut self.data) {
				Some(v) => {
					for p in v {
						new_projectiles.push(p);
					}
				},
				None => {}
			}
		}
		
		for d in &self.data.projectiles {
			
			for p in d.update(&args,&self.data,&self.players) {
				new_projectiles.push(p);
			}
			
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

