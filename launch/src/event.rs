use bevy::ecs::event::Event;


#[derive(Event, Default)]
pub enum FpsCommand {
	#[default]
	State,
	Close,
	Open,
}


#[derive(Event, Default)]
pub enum RulerCommand {
	#[default]
	State,
	Close,
	Open,
}


#[derive(Event, Default)]
pub enum AudioCommand {
	#[default]
	State,
	Volume(i8),
}

#[derive(Event, Default)]
pub enum MusicCommand {
	#[default]
	State,
	Stop,
	Play,
	Pause,
}

#[derive(Event, Default)]
pub enum InputCommand {
	#[default]
	State,
	None,
	Gamepad,
	Keyboard,
}
