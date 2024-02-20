use bevy::{
	ecs::{
		event::{EventReader, EventWriter},
		system::{Local, ResMut},
	},
	input::{
		keyboard::{Key, KeyCode, KeyboardInput},
		ButtonInput, ButtonState,
	},
	window::ReceivedCharacter,
};

use crate::event::UpdateCharacter;



pub fn setup(
	mut update_character: EventWriter<UpdateCharacter>,
	mut received_character: EventReader<ReceivedCharacter>,
) {
	received_character.clear();
	update_character.send(UpdateCharacter(String::from("")));
}

pub fn cleanup(
	mut update_character: EventWriter<UpdateCharacter>,
	mut received_character: EventReader<ReceivedCharacter>,
) {
	received_character.clear();
	update_character.send(UpdateCharacter(String::from("")));
}

pub fn received(
	mut character: Local<String>,
	mut update_character: EventReader<UpdateCharacter>,
	mut received_character: EventReader<ReceivedCharacter>,
	mut key_code: ResMut<ButtonInput<KeyCode>>,
	mut keyboard_input: EventReader<KeyboardInput>,
) -> (
	String,
	bool,
) {
	let mut enter = false;

	for v in received_character.read() {
		if let Some(char) = v.char.chars().nth(0) {
			if char.is_control() {
				continue;
			}

			character.push(char);
		}
	}

	for v in update_character.read() {
		*character = v.0.to_string();
	}

	for v in keyboard_input.read() {
		if v.state == ButtonState::Released {
			continue;
		}

		match v.logical_key {
			| Key::Escape => {
				if !character.is_empty() {
					key_code.clear();

					character.clear();
				}
			},

			| Key::Backspace | Key::Delete => {
				character.pop();
			},

			| Key::Enter => {
				enter = true;
			},

			| _ => {},
		}
	}

	let value = (
		character.to_string(),
		enter,
	);

	if enter {
		character.clear();
	}

	value
}
