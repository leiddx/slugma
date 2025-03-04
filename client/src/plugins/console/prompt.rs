use bevy::{
	ecs::{component::Component, event::EventReader, query::With, system::Query},
	ui::widget::Text,
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
		**text = format!(
			"$ {}_",
			v.0.to_string()
		);
	}
}
