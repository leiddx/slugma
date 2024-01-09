use bevy::{
	ecs::{
		event::{EventReader, EventWriter},
		system::{Res, ResMut, Resource},
	},
	input::{keyboard::KeyCode, Input},
	window::ReceivedCharacter,
};

use super::event::{TextInputCancelEvent, TextInputDoneEvent, TextInputEvent};

#[derive(Resource, Default)]
pub struct TextInput {
	value: String,
}

impl TextInput {
	pub fn pop(&mut self) -> &Self {
		self.value.pop();

		self
	}

	pub fn push(&mut self, ch: char) -> &Self {
		self.value.push(ch);

		self
	}

	pub fn clear(&mut self) -> &Self {
		self.value.clear();

		self
	}

	pub fn to_string(&mut self) -> String {
		self.value.to_string()
	}
}

pub fn setup(
	input_key: Res<Input<KeyCode>>,
	mut text_input: ResMut<TextInput>,
	mut text_input_event: EventWriter<TextInputEvent>,
	mut text_input_done_event: EventWriter<TextInputDoneEvent>,
	mut text_input_cancel_event: EventWriter<TextInputCancelEvent>,
	mut received_character: EventReader<ReceivedCharacter>,
) {
	if input_key.any_just_pressed([KeyCode::Back, KeyCode::Delete]) {
		text_input.pop();

		text_input_event.send(
			TextInputEvent {
				message: text_input.to_string(),
			},
		);

		return;
	}

	if input_key.just_pressed(KeyCode::Escape) {
		text_input.clear();

		text_input_cancel_event.send(TextInputCancelEvent);

		return;
	}

	if input_key.just_pressed(KeyCode::Return) {
		text_input.clear();

		text_input_done_event.send(TextInputDoneEvent);

		return;
	}

	for v in received_character.read() {
		if v.char.is_control() {
			continue;
		}

		text_input.push(v.char);

		text_input_event.send(
			TextInputEvent {
				message: text_input.to_string(),
			},
		);
	}
}

pub fn cleanup(mut text_input: ResMut<TextInput>) {
	text_input.clear();
}
