use bevy::ecs::event::Event;


#[derive(Event, Default)]
pub enum Input {
	#[default]
	None,
	Gamepad,
	Keyboard,

	State,
}
