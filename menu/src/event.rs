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


#[derive(Debug, Event)]
pub enum Player {
	Forward,
	Retreat,
	ForwardAssault,
	RetreatAssault,
	Stand,
	Squat,
	Crawling,
	Block,
	Tumble,
	Jump,
	Break,
	Launch,
	Active,
	Recovery,
	Stun,
}
