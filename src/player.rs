extern crate piston_window;
extern crate piston;

use piston_window::*;

use projectile::*;
use appdata::*;
use device::*;

pub struct Player {
	pub x: f64,
	pub y: f64,
	
	pub speed_x: f64,
	pub speed_y: f64,
	
	pub time_since_shot: f64,
	
	pub device: Device
}

impl Player {

	fn throw_projectiles(&self, data: &AppData) -> Vec<Projectile> {
	
		let mut dir = [0.0,0.0];
		
		if data.key_is_pressed(Key::Left) {
			dir[0] = -1.0;
		}
		else if data.key_is_pressed(Key::Right){
			dir[0] = 1.0;
		}
			
		if data.key_is_pressed(Key::Up){
			dir[1] = -1.0;
		}
		else if data.key_is_pressed(Key::Down){
			dir[1] = 1.0;
		}
			
		
		let mut ret = Vec::new();
		
		for t in &self.device.projectiles {
			
			ret.push(Projectile::new([self.x,self.y],dir,[self.speed_x,self.speed_y],t.clone()));
		}
		
		ret

	}

    pub fn render(&self, c:&Context, g: &mut G2d, data:&AppData) {

        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
		let transform = c.transform.trans(self.x-25.0 as f64,self.y-25.0);
		rectangle(RED, square, transform, g);
    }

    pub fn update(&mut self, args: &UpdateArgs, data: &AppData) -> Option<Vec<Projectile>> {
		
		const SPEED:f64= 100.0;
		
		if data.key_is_pressed(Key::Left) {
			self.x = self.x - (SPEED*args.dt);
		}
		if data.key_is_pressed(Key::Right) {
			self.x = self.x + (SPEED*args.dt);
		}
		
		const PIXELS_PER_METER:f64 = 10.0;

		//jetpack
		if self.y>0.0 && data.key_is_pressed(Key::Return) {
			self.speed_y -= 10.0;
		}
		
		//add gravity
		match self.y+25.0<(data.height as f64) {
			true => {
				self.speed_y = self.speed_y + 9.81*args.dt*PIXELS_PER_METER;
				self.y += self.speed_y*args.dt;
			},
			false =>  {self.speed_y=0.0;}
		}
		
		self.time_since_shot += args.dt;
		
		if self.time_since_shot > self.device.cooldown && data.key_is_pressed(Key::RShift) {
		
			let projectiles = self.throw_projectiles(data);
			self.time_since_shot = 0.0;
			
			return Some(projectiles);
		}
		
		
		
		return None;
		
		
    }
	
	
}