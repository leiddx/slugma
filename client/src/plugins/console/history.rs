use bevy::{
	ecs::{event::EventWriter, system::ResMut},
	input::{keyboard::KeyCode, ButtonInput},
};

use super::event::PromptRefresh;
use crate::{core::Actuator, plugins::character::event::Update};

pub fn roll_back(
	mut actuator: ResMut<Actuator>,
	mut key_code: ResMut<ButtonInput<KeyCode>>,
	mut prompt_refresh: EventWriter<PromptRefresh>,
	mut character_update: EventWriter<Update>,
) {
	if key_code.just_pressed(KeyCode::ArrowUp) {
		key_code.clear();

		if let Some(message) = actuator.roll_back_history() {
			prompt_refresh.send(PromptRefresh(message.clone()));
			character_update.send(Update(message.clone()));
		}
	}
}

pub fn roll_forward(
	mut actuator: ResMut<Actuator>,
	mut key_code: ResMut<ButtonInput<KeyCode>>,
	mut prompt_refresh: EventWriter<PromptRefresh>,
	mut character_update: EventWriter<Update>,
) {
	if key_code.just_pressed(KeyCode::ArrowDown) {
		let message = if let Some(message) = actuator.roll_forward_history() {
			message.clone()
		}
		else {
			String::from("")
		};

		key_code.clear();

		prompt_refresh.send(PromptRefresh(message.clone()));
		character_update.send(Update(message.clone()));
	}
}
