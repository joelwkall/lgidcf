use std::rc::Rc;

#[derive(RustcDecodable)]
pub struct Device {
	pub name: Option<String>,
	pub cooldown: f64,
	pub projectiles: Vec<Rc<ProjectileTemplate>>
}

#[derive(RustcDecodable)]
pub struct ProjectileTemplate {
	pub number:i32,
	pub color:[f32;4],
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
	pub shape_type: ShapeTypes
}

#[derive(RustcDecodable)]
pub enum ShapeTypes {
	Rectangle,
	Ellipse
}