use bevy::prelude::States;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum ConsoleState {
	#[default]
	Close,
	Open,
}
