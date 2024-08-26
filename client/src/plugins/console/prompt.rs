use bevy::{
	prelude::{Component, EventReader, Query, With},
	text::Text,
};

use super::event::PromptRefresh;

#[derive(Component)]
pub struct Prompt;



pub fn refresh(
	mut prompt: Query<&mut Text, With<Prompt>>,
	mut prompt_refresh: EventReader<PromptRefresh>,
) {
	let mut text = prompt.single_mut();

	for v in prompt_refresh.read() {
		text.sections[0].value = format!(
			"$ {}_",
			v.0.to_string()
		);
	}
}
