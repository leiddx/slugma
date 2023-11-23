use bevy::prelude::Resource;
use bevy::prelude::States;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum Game {
	#[default]
	Playing,
	GameOver,
}
