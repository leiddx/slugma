use bevy::{
	ecs::event::{EventReader, EventWriter},
	input::{
		gamepad::{GamepadButtonInput, GamepadButtonType},
		keyboard::{KeyCode, KeyboardInput},
	},
};

use crate::event::Cursor;



pub fn keyboard(mut keyboard_input: EventReader<KeyboardInput>, mut cursor: EventWriter<Cursor>) {
	for v in keyboard_input.read() {
		let v = match v.key_code {
			| KeyCode::KeyW => Some(Cursor::Previous),
			| KeyCode::KeyS => Some(Cursor::Next),
			| KeyCode::KeyA => Some(Cursor::Minus),
			| KeyCode::KeyD => Some(Cursor::Plus),
			| KeyCode::Enter => Some(Cursor::Enter),
			| KeyCode::Backspace => Some(Cursor::Back),

			| _ => None,
		};

		if let Some(v) = v {
			cursor.send(v);
		}
	}
}


pub fn gamepad(
	mut gamepad_button_input: EventReader<GamepadButtonInput>,
	mut cursor: EventWriter<Cursor>,
) {
	for v in gamepad_button_input.read() {
		let v = match v.button.button_type {
			| GamepadButtonType::DPadUp => Some(Cursor::Previous),
			| GamepadButtonType::DPadDown => Some(Cursor::Next),
			| GamepadButtonType::DPadLeft => Some(Cursor::Minus),
			| GamepadButtonType::DPadRight => Some(Cursor::Plus),
			| GamepadButtonType::South => Some(Cursor::Enter),
			| GamepadButtonType::East => Some(Cursor::Back),

			| _ => None,
		};

		if let Some(v) = v {
			cursor.send(v);
		}
	}
}
