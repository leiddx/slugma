use bevy::{
	ecs::{
		event::{EventReader, EventWriter},
		system::{Local, ResMut},
	},
	input::{keyboard::KeyCode, Input},
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
	mut key_code: ResMut<Input<KeyCode>>,
) -> (
	String,
	bool,
) {
	for v in received_character.read() {
		if v.char.is_control() {
			continue;
		}

		character.push(v.char);
	}

	for v in update_character.read() {
		*character = v.0.to_string();
	}

	if key_code.any_just_pressed([KeyCode::Return, KeyCode::NumpadEnter]) {
		let v = (
			character.to_string(),
			true,
		);

		character.clear();
		key_code.clear();

		return v;
	}


	if key_code.any_just_pressed([KeyCode::Back, KeyCode::Delete]) {
		character.pop();

		key_code.clear();
	}

	if key_code.just_pressed(KeyCode::Escape) && !character.is_empty() {
		character.clear();

		key_code.clear();
	}

	(
		character.to_string(),
		false,
	)
}
