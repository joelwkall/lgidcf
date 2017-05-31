


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

	pub fn new(map_size: [f64;2], window_size: [f64;2]) -> App {
	
		
	
		//devices
		let paths = fs::read_dir("devices").unwrap();
		let mut devices = Vec::new();
		for path in paths {
			let mut data = String::new();
			let mut f = File::open(path.unwrap().path()).unwrap();
			f.read_to_string(&mut data).unwrap();
			let d: Device = decode(&data).unwrap();
			devices.push(d);
		}
		
		
		let app_data = AppData::new(map_size, window_size,devices);
		
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
		
		let space = map_size[0] / (s.players.len() as f64+1.0);
		
		for (i,p) in s.players.iter().enumerate() {
		
			let player = Player {
				x:space*(i as f64 + 1.0),
				y:map_size[1]/2.0,
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
				switch_was_pressed:false,
                shape: Shape {
                    shape_type: ShapeTypes::Ellipse,
                    width:100.0,
                    height:100.0,
                    color:p.clone().color
                }
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
		
        //render projectiles
		for d in &self.data.projectiles {
			d.render(&c,g,&self.data);
		}
		
		//render players
		for p in &self.players {
			p.render(&c,g,&self.data,font);
		}
		
		//ground
        let ground = Shape {
            color:[0.0,0.0,0.0,1.0],
            width:self.data.map_size[0],
            height:10.0,
            shape_type:ShapeTypes::Rectangle
        };

        ground.render(&c,g,self.data.map_size[0]/2.0,self.data.map_size[1]-5.0,0.0,&self.data);
		
        //debug info
        let mut text = Text::new(10);
		text.color = [0.0, 0.0, 1.0, 1.0];
		text.draw(&format!("Cam pos: {}:{}, Zoom: {}", self.data.camera_pos[0] as i32,self.data.camera_pos[1] as i32,self.data.zoom),
			font,
			&c.draw_state,
			c.trans(self.data.window_size[0]-250.0, 40.0).transform,
			g); 
	
	}

	pub fn update(&mut self, args: &UpdateArgs) {
		
	
		let mut new_projectiles:Vec<Projectile> = Vec::new();
		
        let mut leftmost_player_x = self.data.map_size[0];
        let mut rightmost_player_x = 0.0;
        let mut top_player_y = self.data.map_size[1];
        let mut bottom_player_y = 0.0;

        //find the player max min pos
		for p in &mut self.players {
			match p.update(&args,&mut self.data) {
				Some(v) => {
					for p in v {
						new_projectiles.push(p);
					}
				},
				None => {}
			}

            if p.x < leftmost_player_x {
                leftmost_player_x = p.x;
            }
            if p.x > rightmost_player_x {
                rightmost_player_x = p.x;
            }
            if p.y < top_player_y {
                top_player_y = p.y;
            }
            if p.y > bottom_player_y {
                bottom_player_y = p.y;
            }
		}
		
		for d in &self.data.projectiles {
			
			for p in d.update(&args,&self.data,&self.players) {
				new_projectiles.push(p);
			}
			
		}

		//self.data.objects = newObjects;
		self.data.projectiles = new_projectiles;


        //find middle point
        let x = (rightmost_player_x + leftmost_player_x)/2.0;
        let y = (bottom_player_y + top_player_y)/2.0;

        //pythagorean distance
        let max_distance = ((rightmost_player_x - leftmost_player_x).powi(2)+(bottom_player_y - top_player_y).powi(2)).sqrt();
        self.data.zoom = 500.0/max_distance;

        if self.data.zoom > 5.0 {
            self.data.zoom = 5.0;
        }

        //set camera position with its middle point on the middle point
        self.data.camera_pos = [(x-(self.data.window_size[0]/self.data.zoom)/2.0),y-(self.data.window_size[1]/self.data.zoom)/2.0];

        //fix boundary issues
        if self.data.camera_pos[0]<0.0 {
            self.data.camera_pos[0] = 0.0;
        }

        if self.data.camera_pos[0]+(self.data.window_size[0]/self.data.zoom)>self.data.map_size[0] {
            self.data.camera_pos[0] = self.data.map_size[0]-(self.data.window_size[0]/self.data.zoom);
        }

        if self.data.camera_pos[1]<0.0 {
            self.data.camera_pos[1] = 0.0;
        }

        if self.data.camera_pos[1]+(self.data.window_size[1]/self.data.zoom)>self.data.map_size[1] {
            self.data.camera_pos[1] = self.data.map_size[1]-(self.data.window_size[1]/self.data.zoom);
        }
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

