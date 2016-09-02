use std::rc::Rc;

use piston_window::*;
use gfx_graphics::GlyphCache;
use gfx_device_gl::Factory;
use gfx_device_gl::Resources;

use projectile::*;
use appdata::*;
use device::*;
use settings::*;

pub struct Player {
	
	pub index: i32,
	pub name: String,
	
	pub x: f64,
	pub y: f64,
	
	pub speed_x: f64,
	pub speed_y: f64,
	
	pub health: f64,
	
	pub time_since_shot: f64,
	
	pub current_device: i32,
	pub jetpack: Rc<ProjectileTemplate>,
	pub settings: Rc<PlayerSettings>,
	
	pub dir: [f64;2],
	
	pub switch_was_pressed: bool
}

impl Player {

	fn throw_projectiles(&self,data: &AppData) -> Vec<Projectile> {
	
		
		
		let mut ret = Vec::new();
		
		let angle = (self.dir[1]).atan2(self.dir[0]);
		let speed =(self.speed_x*self.speed_x + self.speed_y*self.speed_y).sqrt();
		
		for t in &self.get_current_device(data).projectiles {
			for _ in 0..t.number {
				ret.push(Projectile::new([self.x,self.y],angle,speed,t.clone(),self.index));
			}
		}
		
		ret

	}

	pub fn render(&self, c:&Context, g: &mut G2d,data: &AppData, font: &mut GlyphCache<Resources,Factory>) {

		if self.health <= 0.0 {
			return
		}
	
		let square = rectangle::square(0.0, 0.0, 50.0);
		let transform = c.transform.trans(self.x-25.0,self.y-25.0);
		rectangle(self.settings.color, square, transform, g);
		
		let life_bar = rectangle::square(0.0,0.0,100.0); //1px square
		let transform = c.transform.trans(20.0,(self.index as f64 +1.0)*20.0).scale(self.health/100.0,0.1);
		rectangle(self.settings.color, life_bar, transform, g);
		
		let name = match self.settings.name {
			None => {"Noname".to_string()},
			Some(ref n) => {n.to_string()}
		};
		
		//display name
		let mut text = Text::new(10);
		text.color = self.settings.color;
		text.draw(&format!("{}", name),
			font,
			&c.draw_state,
			c.trans(20.0, (self.index as f64 + 1.0)*20.0).transform,
			g); 
		  
		  
		//display current weapon
		//TODO: only show it for a few seconds after switch
		text.color = [1.0, 1.0, 1.0, 1.0];

		let device_name = match self.get_current_device(data).name {
			None => {"Unknown".to_string()},
			Some(ref n) => {n.to_string()}
		};
		
		text.draw(&device_name,
			font,
			&c.draw_state,
			c.trans(self.x-25.0, self.y-30.0).transform,
			g); 
		
	
	}
	
	fn get_current_device<'a>(&'a self, data: &'a AppData) -> &Device {
	
		let ref ret = data.devices[self.current_device as usize];
		
		ret
	
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
	
		//TODO better weapon switching
		if data.key_is_pressed(self.settings.key_switch_weapon) && !self.switch_was_pressed {
			self.current_device+=1;
			
			if self.current_device >= data.devices.len() as i32 {
				self.current_device = 0;
			}
			
			self.switch_was_pressed = true;
		}
		else if !data.key_is_pressed(self.settings.key_switch_weapon) {
			self.switch_was_pressed = false;
		}
		
		
		const PIXELS_PER_METER:f64 = 10.0;
		
		let mut ret = Vec::new();

		//jetpack
		if self.y > 0.0 && data.key_is_pressed(self.settings.key_jetpack) {
			self.speed_y -= 5.0;

			for _ in 0..self.jetpack.number {
				ret.push(Projectile::new([self.x,self.y],3.14159*0.5,0.0,self.jetpack.clone(),self.index));
				
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
		
		//add gravity
		self.y += self.speed_y*args.dt;
		
		
		//shoot device
		self.time_since_shot += args.dt;
		if self.time_since_shot > self.get_current_device(data).cooldown && data.key_is_pressed(self.settings.key_fire) {
		
			let projectiles = self.throw_projectiles(data);
			self.time_since_shot = 0.0;
			
			ret.extend(projectiles);
		}
		
		
		//get hit
		for p in &data.projectiles {
		
			match p.template.damage {
				Some(d) => {
					if 
						self.index != p.owner_index &&					//only take damage from other players projectiles
						p.x - p.template.shape.width/2.0 < self.x+25.0 &&
						p.x + p.template.shape.width/2.0 > self.x-25.0 &&
						p.y - p.template.shape.height/2.0 < self.y+25.0 &&
						p.y + p.template.shape.height/2.0 > self.y-25.0

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