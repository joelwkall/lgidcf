extern crate rand;

use std::rc::Rc;
use rand::Rng;

use piston_window::*;

use obstacle::Obstacle;
use player::Player;
use appdata::*;
use device::*;

pub struct Projectile {

	pub x: f64,
	pub y: f64,
	
	pub speed: f64,
	pub direction: f64,
	
	pub owner_index: i32,
	
	pub template: Rc<ProjectileTemplate>,
	
	age:f64
}

impl Projectile {

    //TODO: make it work with speedX, speedY again, much easier
	pub fn new(pos:[f64;2],angle:f64,speed:f64,template:Rc<ProjectileTemplate>,owner:i32) -> Projectile {

		//add spread
		let mut new_angle = angle;
		if template.spread.unwrap_or(0.0) > 0.0 {
			let mut rng = rand::thread_rng();
			new_angle += rng.gen_range::<f64>(-template.spread.unwrap_or(0.0),template.spread.unwrap_or(0.0));
		}
	
		let ret = Projectile {
			x:pos[0],
			y:pos[1],
			speed:speed*template.inherit_speed.unwrap_or(0.0) + template.initial_speed.unwrap_or(0.0),
			direction:new_angle,
			age:0.0,
			template:template,
			owner_index:owner
		};
		
		ret
	}

	pub fn render(&self, c:&Context, g: &mut G2d,data:&AppData) {

        self.template.shape.render(&c,g,self.x,self.y,self.direction,&data);
	
	}
	
    //TODO: could be implemented as 4 virtual obstacles around the map to avoid code duplication
	fn check_border_collision(&self, ret: &mut Projectile, data: &AppData) -> Vec<&ProjectileEvent>
	{
		const HALFTURN:f64 = 3.14159;
		const WHOLETURN:f64 = HALFTURN*2.0;
	
		let mut vec = Vec::new();
	
		let mut trigger_border_collision = false;
		
		//TODO: fix this weird algorithm so it doesnt need to move 1px
		let reduction = 1.0-self.template.friction.unwrap_or(0.0);
		if self.x <= 0.0
		{
			ret.direction = HALFTURN - ret.direction; //mirror angle along y axis
			ret.speed = ret.speed*reduction;
			ret.x = 1.0;
			
			trigger_border_collision=true;
		}
		
		if self.x >= (data.map_size[0] as f64)
		{
			ret.direction = HALFTURN - ret.direction; //mirror angle along y axis
			ret.speed = ret.speed*reduction;
			ret.x = (data.map_size[0] as f64)-1.0;
			
			trigger_border_collision=true;
		}
	
		if self.y <= 0.0
		{
			ret.direction = -ret.direction; //mirror angle along x axis
			ret.speed = ret.speed*reduction;
			ret.y = 1.0;
			
			trigger_border_collision=true;
		}
		
		if self.y >= (data.map_size[1] as f64)
		{
			ret.direction = -ret.direction; //mirror angle along x axis
			ret.speed = ret.speed*reduction;
			ret.y = (data.map_size[1] as f64)-1.0;
			
			trigger_border_collision=true;
		}
		
		//normalize angle
		while ret.direction < 0.0 {
			ret.direction += WHOLETURN;
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
	
	fn check_player_collision(&self, players: &Vec<Player>) -> Vec<&ProjectileEvent>
	{
	
		let mut vec = Vec::new();
	
		let mut trigger_collision = false;
		
		for p in players {
		
			if 
				p.health > 0.0 && //ignore dead players
				self.owner_index != p.index &&					//only match non-owner players
				self.x - self.template.shape.width/2.0 < p.x+p.health/2.0 &&
				self.x + self.template.shape.width/2.0 > p.x-p.health/2.0 &&
				self.y - self.template.shape.height/2.0 < p.y+p.health/2.0 &&
				self.y + self.template.shape.height/2.0 > p.y-p.health/2.0

			{
				trigger_collision=true;
			}
		}
		
		if trigger_collision {
		
			for e in &self.template.events {
				
				match e.event_type {
					ProjectileEventTypes::PlayerCollision => vec.push(e),
					_ => {}
				}
			
			}
		
		}
		
		vec
	}

    fn check_obstacle_collision(&self, args: &UpdateArgs, ret: &mut Projectile, obstacles: &Vec<Obstacle>) -> Vec<&ProjectileEvent>
	{
        const HALFTURN:f64 = 3.14159;
		const WHOLETURN:f64 = HALFTURN*2.0;
	
		let mut vec = Vec::new();
	
		let mut trigger_collision = false;
		
        let reduction = 1.0-self.template.friction.unwrap_or(0.0);

        let prev_x = self.x - self.speed*self.direction.cos()*args.dt;
		let prev_y = self.y - self.speed*self.direction.sin()*args.dt;

		for o in obstacles {
		
            //check if we hit (we are inside)
            if 
                self.x - self.template.shape.width/2.0 < o.x+o.width/2.0 &&
                self.x + self.template.shape.width/2.0 > o.x-o.width/2.0 &&
                self.y - self.template.shape.height/2.0 < o.y+o.height/2.0 &&
                self.y + self.template.shape.height/2.0 > o.y-o.height/2.0 
            {
                trigger_collision=true;
                ret.speed = ret.speed*reduction;

                //different bouncing depending on where we hit
                //TODO: fix this weird algorithm so it doesnt need to move 1px

                /*try to find from which section it came:
                
                \   3  /
                 \____/
               2 |    |   1
                 |____|
                 /    \
                /   4  \


                */
                //right edge
		        //if
		        //{
			       // ret.direction = HALFTURN - ret.direction; //mirror angle along y axis
			       // ret.x = ret.x+1.0;
		        //}
          //      //left edge
		        //else if
		        //{
			       // ret.direction = HALFTURN - ret.direction; //mirror angle along y axis
			       // ret.x = ret.x-1.0;
		        //}
	         //   //top edge
		        //else if
		        //{
			       // ret.direction = -ret.direction; //mirror angle along x axis
			       // ret.y = ret.y-1.0;
		        //}
		        ////bottom edge
		        //else if
		        //{
			       // ret.direction = -ret.direction; //mirror angle along x axis
			       // ret.y = ret.y+1.0;
		        //}
            }

		}

        //normalize angle
		while ret.direction < 0.0 {
			ret.direction += WHOLETURN;
		}
		
		if trigger_collision {
		
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
	
		if self.speed.abs() < STATIONARY_THRESHHOLD {
			for e in &self.template.events {
				
				match e.event_type {
					ProjectileEventTypes::Stopped =>vec.push(e),
					_ => {}
				}
			}
		}
		
		vec
	}
	
	fn check_timers(&self, args: &UpdateArgs) -> Vec<&ProjectileEvent>
	{
		let mut vec = Vec::new();
		
		//TODO: add support for multiple triggers for the same action
		for e in &self.template.events {
			
			match e.event_type {
				ProjectileEventTypes::Timer =>{
				
					let time = e.time.unwrap_or(0.0);
				
					match e.repeat.unwrap_or(false) {
						false => {
							if self.age >= time && self.age - args.dt < time {
								vec.push(e);
							}
						},
						true => {
						
							let has_started = match e.start_at {
								None => true,
								Some(t) => t < self.age
							};
							
							let has_ended = match e.end_at {
								None => false,
								Some(t) => t < self.age
							};
							
							if has_started && !has_ended {
						
								let cycles = (self.age/time).floor();
								let last = cycles*time;
								
								if self.age >= last && self.age-args.dt < last {
									vec.push(e);
								}
							}
						}
					}
				},
				_ => {}
			}
		}

		vec
	
	}

	pub fn update(&self, args: &UpdateArgs, data: &AppData, players: &Vec<Player>) -> Vec<Projectile> {
		
		let mut ret = Projectile {
				age:self.age+args.dt,
				template: self.template.clone(),
			..*self};
			
		let mut triggered_events = Vec::new();
		
		for e in self.check_border_collision(&mut ret,data) {
			triggered_events.push(e);
		}
		
		for e in self.check_player_collision(players) {
			triggered_events.push(e);
		}

        for e in self.check_obstacle_collision(args, &mut ret, &data.obstacles) {
			triggered_events.push(e);
		}
		
		for e in self.check_stationary() {
			triggered_events.push(e);
		}
		
		for e in self.check_timers(args) {
			triggered_events.push(e);
		}

		
		const PIXELS_PER_METER:f64 = 10.0;
		
		//add gravity
		let gravity = 9.81*args.dt*PIXELS_PER_METER*ret.template.gravity.unwrap_or(1.0);
		
		
		let dx = ret.speed*ret.direction.cos(); //calculate speed in x direction
		let dy = ret.speed*ret.direction.sin() + gravity; //calculate y speed and add gravity
		ret.direction = (dy).atan2(dx); //set new direction based on dx and dy
		ret.speed += gravity*ret.direction.sin(); // add gravity to speed
		
		//add acceleration
		//TODO: acceleration should not affect speed gained by gravity?
		ret.speed += ret.template.acceleration.unwrap_or(0.0)*args.dt*PIXELS_PER_METER;
		
		//calculate new position
		ret.x = ret.x + ret.speed*ret.direction.cos()*args.dt;
		ret.y = ret.y + ret.speed*ret.direction.sin()*args.dt;
		
	
		let mut return_vec = Vec::new();
		let mut has_died = false;
		for e in triggered_events {
		
			match &e.spawn_projectiles {
			
				&Some(ref vec) => {
				
					for p in vec {
			
						for _ in 0..p.number {
							return_vec.push(Projectile::new([ret.x,ret.y],ret.direction,ret.speed,p.clone(),ret.owner_index)); //use updated projectile values
						}
					}
				},
				_ => {}
			}
		
			match e.die {
				Some(true) => has_died = true,
				_ => {}
			}
		}
		
		if !has_died {
			return_vec.push(ret);
		}
	
		return_vec
	}
}