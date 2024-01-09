use bevy::ecs::event::Event;

#[derive(Event)]
pub struct TextInputEvent {
	pub message: String,
}

#[derive(Event)]
pub struct TextInputDoneEvent;

#[derive(Event)]
pub struct TextInputCancelEvent;
