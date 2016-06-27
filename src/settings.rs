use std::rc::Rc;

use piston_window::*;

#[derive(RustcDecodable)]
pub struct Settings {
	pub players: Vec<Rc<PlayerSettings>>
}

#[derive(RustcDecodable)]
pub struct PlayerSettings {
	pub name: Option<String>,
	pub color:[f32;4],
	pub key_up: Key,
	pub key_down: Key,
	pub key_left: Key,
	pub key_right: Key,
	pub key_fire: Key,
	pub key_jetpack: Key
}