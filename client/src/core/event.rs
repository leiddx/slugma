use bevy::ecs::event::Event;
use regex::Regex;

use super::Level;



#[derive(Event)]
pub(crate) struct Refresh;


#[derive(Event)]
pub(crate) struct Execute(pub String);

impl Execute {
	pub fn new(value: String) -> Self {
		let value = Regex::new(r"^`+")
			.unwrap()
			.replace_all(
				&value, "",
			)
			.trim()
			.to_string();

		Self(value)
	}

	pub fn trim(&self) -> &str {
		self.0.trim()
	}
}

#[derive(Debug, Event)]
pub(crate) enum Feedback {
	Append(
		String,
		Level,
	),
	Replace(
		String,
		Level,
	),
}
