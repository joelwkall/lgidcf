use std::rc::Rc;

use piston_window::*;

use appdata::*;

#[derive(RustcDecodable)]
pub struct Device {
	pub name: Option<String>,
	pub cooldown: f64,
	pub projectiles: Vec<Rc<ProjectileTemplate>>
}

#[derive(RustcDecodable)]
pub struct ProjectileTemplate {
	pub number:i32,
	pub initial_speed:Option<f64>,
	pub inherit_speed:Option<f64>,
	pub acceleration:Option<f64>,
	pub gravity:Option<f64>,
	pub spread:Option<f64>,
	pub friction:Option<f64>,
	pub events: Vec<ProjectileEvent>,
	pub damage:Option<f64>,
	pub shape:Shape
}

#[derive(RustcDecodable)]
pub struct ProjectileEvent {
	pub event_type: ProjectileEventTypes,
	pub time: Option<f64>,
	pub start_at: Option<f64>,
	pub end_at: Option<f64>,
	pub repeat: Option<bool>,
	pub spawn_projectiles: Option<Vec<Rc<ProjectileTemplate>>>,
	pub die:Option<bool>
}


#[derive(RustcDecodable)]
pub enum ProjectileEventTypes {
	BorderCollision,
	PlayerCollision,
	Stopped,
	Timer
}

#[derive(RustcDecodable)]
pub struct Shape {
	pub width: f64,
	pub height:f64,
    pub color:[f32;4],
	pub shape_type: ShapeTypes
}

impl Shape {

    pub fn render(&self, c:&Context, g: &mut G2d, x: f64, y: f64, dir: f64, data:&AppData) {
	
        //TODO dont draw if outside screen

        let x = (x-data.camera_pos[0])*data.zoom;
        let y = (y-data.camera_pos[1])*data.zoom;

		let transform = c.transform.trans(x,y).rot_rad(dir);
		
		let rect = rectangle::centered([0.0,0.0,(self.width/2.0)*data.zoom,(self.height/2.0)*data.zoom]);
		
		//TODO: create the shape once, and draw multiple times
        //might be hard with differing resolution
		match self.shape_type {
			ShapeTypes::Rectangle => { 
			
				Rectangle::new(self.color)
					.draw(rect, &Default::default(), transform, g);
			},
			ShapeTypes::Ellipse => { 
			
                let resolution = data.zoom*2.0*self.width.max(self.height).sqrt();

				Ellipse::new(self.color)
					.resolution(resolution as u32)
					.draw(rect, &Default::default(), transform, g);
			}
		}
	}

    //TODO: move collision detection here
}



#[derive(RustcDecodable)]
pub enum ShapeTypes {
	Rectangle,
	Ellipse
}