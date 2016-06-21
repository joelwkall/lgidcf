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
	pub size:f64,
	pub color:[f32;4],
	pub speed:f64,
	pub spread:f64,
	pub bounce:f64,
	pub friction:f64,
	pub events: Vec<ProjectileEvent>
}

#[derive(RustcDecodable)]
pub struct ProjectileEvent {
	pub event_type: ProjectileEventTypes,
	
	pub spawn_projectiles: Option<Vec<Rc<ProjectileTemplate>>>,
	pub die:Option<bool>,
	pub damage:Option<f64>
}


#[derive(RustcDecodable)]
pub enum ProjectileEventTypes {
	BorderCollision,
	PlayerCollision,
	Stopped
}