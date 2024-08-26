use bevy::ecs::event::Event;


#[derive(Event, Default)]
pub enum Fps {
	#[default]
	Close,
	Open,

	State,
}
