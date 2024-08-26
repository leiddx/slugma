use bevy::state::state::States;


#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum Input {
	#[default]
	None,
	Gamepad,
	Keyboard,
}
