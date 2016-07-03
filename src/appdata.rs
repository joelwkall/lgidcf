
use std::collections::HashMap;

use piston_window::*;

use projectile::*;

pub struct AppData {
	pub projectiles: Vec<Projectile>,
	pub width: u32,
	pub height: u32,
	
	pub mouse_x:u32,
	pub mouse_y:u32,
	pub mouse_pressed: bool,
	
	buttons: HashMap<Key,bool>
	
}

impl AppData {

	pub fn new(width:u32,height:u32) -> AppData { 
		AppData{
			projectiles:Vec::new(),
			width:width,
			height:height,
			mouse_x:0,
			mouse_y:0,
			mouse_pressed:false,
			buttons:HashMap::new()
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