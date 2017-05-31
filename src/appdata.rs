extern crate rand;

use std::collections::HashMap;
use piston_window::*;
use rand::Rng;
use obstacle::*;
use projectile::*;
use device::*;

pub struct AppData {
	pub projectiles: Vec<Projectile>,
    pub obstacles: Vec<Obstacle>,
	pub devices: Vec<Device>,
	
	pub mouse_x:u32,
	pub mouse_y:u32,
	pub mouse_pressed: bool,

    pub map_size: [f64;2],
    pub window_size: [f64;2],

    pub camera_pos: [f64;2],
    pub zoom: f64,
	
	buttons: HashMap<Key,bool>
	
}

impl AppData {

	pub fn new(map_size: [f64;2], window_size: [f64;2], devices:Vec<Device>) -> AppData { 

        let mut obstacles = Vec::new();

        let mut rng = rand::thread_rng();
		
        for _ in 0..30 {

            let width = rng.gen_range::<f64>(100.0,300.0);
            let height = rng.gen_range::<f64>(20.0,40.0);

            let x = rng.gen_range::<f64>(0.0,map_size[0] as f64);
            let y = rng.gen_range::<f64>(0.0,map_size[1] as f64);

            let o = Obstacle::new([x,y],[width,height]);

            obstacles.push(o);
        }
           
		AppData{
			projectiles:Vec::new(),
            obstacles:obstacles,
			devices:devices,
			map_size:map_size,
			window_size:window_size,
			mouse_x:0,
			mouse_y:0,
			mouse_pressed:false,
			buttons:HashMap::new(),
            camera_pos:[0.0,0.0],
            zoom:1.0
		}
	}
	
	pub fn key_is_pressed(&self,k:Key) -> bool {
		match self.buttons.get(&k) {
			Some(b) => return *b,
			None => return false
		}
	}
	
	pub fn handle_button_pressed(&mut self, button: Button) {

		match button {
			Button::Keyboard(key) => {self.buttons.insert(key,true);},
			Button::Mouse(_) => {self.mouse_pressed=true;},
			Button::Controller(_) => {}
		}

	}
	
	pub fn handle_button_released(&mut self, button: Button) {

		match button {
			Button::Keyboard(key) => {self.buttons.insert(key,false);},
			Button::Mouse(_) => {self.mouse_pressed=false},
			Button::Controller(_) => {}
		}

	}
}