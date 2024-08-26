use bevy::state::state::States;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum Fps {
	#[default]
	Off,
	On,
}
