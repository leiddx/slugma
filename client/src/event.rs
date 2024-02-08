use bevy::ecs::event::Event;
use regex::Regex;

use crate::command::RecordLevel;

#[derive(Event, Debug)]
pub struct UpdateCharacter(pub String);

#[derive(Event)]
pub struct PromptRefresh(pub String);

#[derive(Event)]
pub struct PanelRefresh;

#[derive(Event)]
pub struct Execute(pub String);

impl Execute {
	pub fn new(value: String) -> Self {
		let value = Regex::new(r"^`+")
			.unwrap()
			.replace_all(
				&value, "",
			)
			.trim()
			.to_string();

		Execute(value)
	}

	pub fn trim(&self) -> &str {
		self.0.trim()
	}
}

#[derive(Debug, Event)]
pub enum Feedback {
	Append(
		String,
		RecordLevel,
	),
	Replace(
		String,
		RecordLevel,
	),
}
