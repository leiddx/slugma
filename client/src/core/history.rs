use bevy::prelude::Resource;

#[derive(Resource, Default, Debug)]
pub struct History {
	index: Option<usize>,
	value: Vec<String>,
}

impl History {
	pub fn push(&mut self, value: &str) {
		let value = value.trim();

		self.index = None;

		self.value
			.push(value.to_string());
	}

	pub fn roll_back(&mut self) -> Option<&String> {
		let index = if let Some(index) = self.index {
			index.saturating_sub(1)
		}
		else {
			self.value
				.len()
				.saturating_sub(1)
		};


		self.index = Some(index);

		self.value.get(index)
	}

	pub fn roll_forward(&mut self) -> Option<&String> {
		let length = self.value.len();

		let index = if let Some(index) = self.index {
			index.saturating_add(1)
		}
		else {
			length
		};

		if index > length.saturating_sub(1) {
			self.index = None;

			None
		}
		else {
			self.index = Some(index);

			self.value.get(index)
		}
	}
}
