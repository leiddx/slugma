use bevy;
use bevy::prelude::Entity;
use bevy::prelude::Resource;
use bevy::time::Timer;


#[derive(Resource, Default)]
pub struct Player {
	pub x:             usize,
	pub y:             usize,
	pub move_cooldown: Timer,
	pub entity:        Option<Entity>,
}
