extern crate piston_window;
extern crate piston;
extern crate rand;

use std::rc::Rc;
use rand::Rng;

use piston_window::*;

use appdata::*;
use device::*;



pub struct Projectile {

	pub x: f64,
	pub y: f64,
	
	pub speed_x: f64,
	pub speed_y: f64,
	
	age:f64,
	template: Rc<ProjectileTemplate>
}


impl Projectile {
    


	pub fn new(pos:[f64;2],dir:[f64;2],speed:[f64;2],template:Rc<ProjectileTemplate>) -> Projectile {

	
		//normalize direction vector
		let dx = dir[0];
		let dy = dir[1];
		let mut angle = (-dy).atan2(dx);
		
		if template.spread > 0.0 {
			let mut rng = rand::thread_rng();
			angle += rng.gen_range::<f64>(-template.spread,template.spread)
		}
		
		let dx = angle.cos();
		let dy = -angle.sin();
	
		Projectile {
			x: pos[0],
			y:pos[1],
			speed_x:speed[0] + dx*template.speed,
			speed_y:speed[1] + dy*template.speed,
			age:0.0,
			template:template
		}
	}

	pub fn render(&self, c:&Context, g: &mut G2d) {

        let square = rectangle::square(0.0, 0.0, self.template.size);
		let transform = c.transform.trans(self.x-self.template.size/2.0,self.y-self.template.size/2.0);
		rectangle(self.template.color, square, transform, g);

    }
	
	fn check_border_collision(&self, ret: &mut Projectile, data: &AppData) -> Vec<&ProjectileEvent>
	{
	
		let mut vec = Vec::new();
	
		let mut trigger_border_collision = false;
		
		//TODO: fix this weird algorithm so it doesnt need to move 1px
		let reduction = 1.0-self.template.friction;
		if self.x <= 0.0
		{
			ret.speed_x = -ret.speed_x*self.template.bounce;
			ret.speed_y = ret.speed_y*reduction;
			ret.x = 1.0;
			
			trigger_border_collision=true;
		}
		
		if self.x >= (data.width as f64)
		{
			ret.speed_x = -ret.speed_x*self.template.bounce;
			ret.speed_y = ret.speed_y*reduction;
			ret.x = (data.width as f64)-1.0;
			
			trigger_border_collision=true;
		}
	
		if self.y <= 0.0
		{
			ret.speed_y = -ret.speed_y*self.template.bounce;
			ret.speed_x = ret.speed_x*reduction;
			ret.y = 1.0;
			
			trigger_border_collision=true;
		}
		
		if self.y >= (data.height as f64)
		{
			ret.speed_y = -ret.speed_y*self.template.bounce;
			ret.speed_x = ret.speed_x*reduction;
			ret.y = (data.height as f64)-1.0;
			
			trigger_border_collision=true;
		}
		
		if trigger_border_collision {
		
			for e in &self.template.events {
				
				match e.event_type {
					ProjectileEventTypes::BorderCollision => vec.push(e),
					_ => {}
				}
			
			}
		
		}
		
		vec
	}
	
	fn check_stationary(&self) -> Vec<&ProjectileEvent>
	{
		const STATIONARY_THRESHHOLD : f64 = 1.0;
		let mut vec = Vec::new();
	
		if self.speed_x.abs() < STATIONARY_THRESHHOLD && self.speed_y.abs() < STATIONARY_THRESHHOLD {
			for e in &self.template.events {
				
				match e.event_type {
					ProjectileEventTypes::Stopped =>vec.push(e),
					_ => {}
				}
			}
		}
		
		vec
	
	}

    pub fn update(&self, args: &UpdateArgs, data: &AppData) -> Vec<Projectile> {
		
		let mut ret = Projectile {
				age:self.age+args.dt,
				template: self.template.clone(),
			..*self};
			
		let mut triggered_events = Vec::new();
		
		
	
		
		for e in self.check_border_collision(&mut ret,data) {
			triggered_events.push(e);
		}
		
		for e in self.check_stationary() {
			triggered_events.push(e);
		}
		
		

		
		const PIXELS_PER_METER:f64 = 10.0;
		
		
		ret.speed_y = ret.speed_y + 9.81*args.dt*PIXELS_PER_METER; //add gravity
		ret.x = ret.x + ret.speed_x*args.dt;
		ret.y = ret.y + ret.speed_y*args.dt;
		
	
		let mut return_vec = Vec::new();
		let mut has_died = false;
		for e in triggered_events {
		
			for p in &e.spawn_projectiles {
			
				for _ in 0..p.number {
					return_vec.push(Projectile::new([ret.x,ret.y],[ret.speed_x,ret.speed_y],[ret.speed_x,ret.speed_y],p.clone())); //use updated projectile values
				}
			}
		
			if e.die {
				has_died = true;
			}
		}
		
		if !has_died {
			return_vec.push(ret);
		}
	

		return_vec
    }
}