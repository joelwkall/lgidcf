extern crate piston_window;
extern crate piston;

use std::rc::Rc;

use piston_window::*;

use projectile::*;
use appdata::*;
use device::*;
use settings::*;

pub struct Player {
	
	pub index: i32,
	
	pub x: f64,
	pub y: f64,
	
	pub speed_x: f64,
	pub speed_y: f64,
	
	pub health: f64,
	
	pub time_since_shot: f64,
	
	pub device: Rc<Device>,
	pub jetpack: Rc<ProjectileTemplate>,
	pub settings: Rc<PlayerSettings>,
	
	pub dir: [f64;2]
}

impl Player {

	fn throw_projectiles(&self) -> Vec<Projectile> {
	
		
		
		let mut ret = Vec::new();
		
		for t in &self.device.projectiles {
			for _ in 0..t.number {
				ret.push(Projectile::new([self.x,self.y],self.dir,[self.speed_x,self.speed_y],t.clone(),self.index));
			}
		}
		
		ret

	}

    pub fn render(&self, c:&Context, g: &mut G2d) {

		if self.health <= 0.0 {
			return
		}
	
        let square = rectangle::square(0.0, 0.0, 50.0);
		let transform = c.transform.trans(self.x-25.0,self.y-25.0);
		rectangle(self.settings.color, square, transform, g);
		
		let life_bar = rectangle::square(0.0,0.0,100.0); //1px square
		let transform = c.transform.trans(20.0,(self.index as f64 +1.0)*20.0).scale(self.health/100.0,0.1);
		rectangle(self.settings.color, life_bar, transform, g);
			
    }

    pub fn update(&mut self, args: &UpdateArgs, data: &AppData) -> Option<Vec<Projectile>> {
		
		
		
		if self.health <= 0.0 {
			return None
		}
		
		const SPEED:f64= 100.0;
		
		//go left and right
		if data.key_is_pressed(self.settings.key_left) {
			self.x = self.x - (SPEED*args.dt);
			self.dir[0] = -1.0;
		}
		else if data.key_is_pressed(self.settings.key_right) {
			self.x = self.x + (SPEED*args.dt);
			self.dir[0] = 1.0;
		}
		else if data.key_is_pressed(self.settings.key_up) || data.key_is_pressed(self.settings.key_down)
		{
			self.dir[0] = 0.0;
		}
		
		//keep track of directionality
		if data.key_is_pressed(self.settings.key_up){
			self.dir[1] = -1.0;
		}
		else if data.key_is_pressed(self.settings.key_down){
			self.dir[1] = 1.0;
		}
		else if data.key_is_pressed(self.settings.key_left) || data.key_is_pressed(self.settings.key_right){
			self.dir[1] = 0.0;
		}
	
		
		
		
		const PIXELS_PER_METER:f64 = 10.0;
		
		let mut ret = Vec::new();

		//jetpack
		if self.y > 0.0 && data.key_is_pressed(self.settings.key_jetpack) {
			self.speed_y -= 10.0;

			for _ in 0..self.jetpack.number {
				ret.push(Projectile::new([self.x,self.y],[0.0,1.0],[0.0,0.0],self.jetpack.clone(),self.index));
				
			}
		}
		
		//add gravity
		self.speed_y += 9.81*args.dt*PIXELS_PER_METER;
			
		
		//floor
		if self.y+25.0 > (data.height as f64) {
			self.speed_y = 0.0;
			self.y = (data.height as f64) - 25.0;
		}

		//ceiling
		if self.y-25.0 < 0.0 {
			self.speed_y = 0.0;
			self.y = 25.0;
		}
		
		if self.x+25.0 > (data.width as f64) {
			self.speed_x = 0.0;
			self.x = (data.width as f64) - 25.0;
		}
		
		if self.x-25.0 < 0.0 {
			self.speed_x = 0.0;
			self.x = 25.0;
		}
		
		self.y += self.speed_y*args.dt;
		
		self.time_since_shot += args.dt;
		
		
		
		if self.time_since_shot > self.device.cooldown && data.key_is_pressed(self.settings.key_fire) {
		
			let projectiles = self.throw_projectiles();
			self.time_since_shot = 0.0;
			
			ret.extend(projectiles);
		}
		
		for p in &data.projectiles {
		
			match p.template.damage {
				Some(d) => {
					if 
						self.index != p.owner_index &&					//only take damage from other players projectiles
						p.x - p.template.size/2.0 < self.x+25.0 &&
						p.x + p.template.size/2.0 > self.x-25.0 &&
						p.y - p.template.size/2.0 < self.y+25.0 &&
						p.y + p.template.size/2.0 > self.y-25.0

					{
						self.health -= d;
					}
				},
				None => {}
			}
			
		}
		
		Some(ret)
		
		
		
    }
	
	
}