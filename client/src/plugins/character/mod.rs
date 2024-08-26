pub mod event;

use bevy::{
	app::App,
	ecs::{
		event::{EventReader, EventWriter},
		system::Local,
	},
	input::keyboard::{Key, KeyboardInput},
};
use event::Update;




pub fn setup(
	mut keyboard_input: EventReader<KeyboardInput>,
	mut update_character: EventWriter<Update>,
) {
	keyboard_input.clear();
	update_character.send(Update(String::from("")));
}

pub fn cleanup(
	mut keyboard_input: EventReader<KeyboardInput>,
	mut update_character: EventWriter<Update>,
) {
	keyboard_input.clear();
	update_character.send(Update(String::from("")));
}

pub fn received(
	mut character: Local<String>,
	mut update_character: EventReader<Update>,
	mut keyboard_input: EventReader<KeyboardInput>,
) -> (
	String,
	bool,
) {
	let mut enter = false;

	for v in update_character.read() {
		*character = v.0.to_string();
	}

	for v in keyboard_input.read() {
		if !v.state.is_pressed() {
			continue;
		}

		match &v.logical_key {
			| Key::Enter => {
				enter = true;
			},

			| Key::Escape => {
				if !character.is_empty() {
					character.clear();
				}
			},

			| Key::Backspace | Key::Delete => {
				character.pop();
			},

			| Key::Space => {
				character.push(' ');
			},

			| Key::Character(input) => {
				let text = input.to_string();

				character.push_str(&text);
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


pub struct Plugin;

impl bevy::app::Plugin for Plugin {
	fn build(&self, app: &mut App) {
		app.add_event::<Update>();
	}
}
