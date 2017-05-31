extern crate rand;

use piston_window::*;

use appdata::*;
use device::*;

pub struct Obstacle {

	pub x: f64,
	pub y: f64,

    pub width: f64,
    pub height: f64,

    shape:Shape
}

impl Obstacle {
	
    //TODO: make it work with speedX, speedY again, much easier
	pub fn new(pos:[f64;2],size:[f64;2]) -> Obstacle {

		let ret = Obstacle {
			x:pos[0],
			y:pos[1],
			width:size[0],
            height:size[1],
            shape:Shape{
                shape_type:ShapeTypes::Rectangle,
                width:size[0],
                height:size[1],
                color:[0.7,0.7,0.7,1.0]
            }
		};
		
		ret
	}

	pub fn render(&self, c:&Context, g: &mut G2d,data:&AppData) {
        self.shape.render(&c,g,self.x,self.y,0.0,&data);
	}
}