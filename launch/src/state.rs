use bevy::ecs::schedule::States;


#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum Fps {
	#[default]
	Off,
	On,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum Ruler {
	#[default]
	Off,
	On,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum Music {
	#[default]
	Stop,
	Play,
	Pause,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameInput {
	#[default]
	None,
	Gamepad,
	Keyboard,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameChapter {
	#[default]
	Launch,
	Menu,
}
