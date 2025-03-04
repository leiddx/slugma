use std::collections::{vec_deque::Iter, VecDeque};

use bevy::ecs::system::Resource;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum Level {
	#[default]
	Info,
	Success,
	Error,
	Warn,
	Display,
}


#[derive(Resource, Default, Debug)]
pub struct Record {
	max: usize,

	value: VecDeque<(
		String,
		Level,
	)>,
}

impl Record {
	pub fn new(max: usize) -> Self {
		Self {
			max,
			..Default::default()
		}
	}

	pub fn clear(&mut self) {
		self.value.clear()
	}

	pub fn push(&mut self, message: &str, level: Level) {
		for v in message.split("\n") {
			self.value.push_back(
				(
					v.to_string(),
					level.clone(),
				),
			);

			if self.value.len() > self.max {
				self.value.pop_front();
			}
		}
	}

	pub fn push_empty(&mut self, quantity: usize) {
		for _ in 0..quantity {
			self.push(
				"",
				Level::Display,
			);
		}
	}

	pub fn replace_last(&mut self, message: &str, level: Level) {
		self.value.pop_back();

		self.push(
			message, level,
		);
	}

	pub fn get(
		&self,
		index: &Option<usize>,
		quantity: &usize,
	) -> Iter<(
		String,
		Level,
	)> {
		let last_index = self.last_index();

		let end = last_index.min(if let Some(v) = index { *v } else { last_index });

		let start = end
			.saturating_add(1)
			.saturating_sub(*quantity);

		if start == end {
			self.value.iter()
		}
		else {
			self.value
				.range(start..=end)
		}
	}

	pub fn len(&self) -> usize {
		self.value.len()
	}

	pub fn last_index(&self) -> usize {
		self.value
			.len()
			.saturating_sub(1)
	}
}
