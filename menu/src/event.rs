use bevy::ecs::event::Event;

#[derive(Debug, Event)]
pub enum Cursor {
	Previous,
	Next,
	Plus,
	Minus,
	Enter,
	Back,
}
