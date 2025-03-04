use bevy::ecs::system::{Resource, SystemId};
use clap::ArgMatches;

#[derive(Resource, Debug)]
pub struct Distribute {
	system:    SystemId,
	arguments: Vec<ArgMatches>,
}

impl Distribute {
	pub fn new(system: SystemId) -> Self {
		Self {
			system,

			arguments: Default::default(),
		}
	}

	pub fn system_id(&self) -> &SystemId {
		&self.system
	}

	pub fn read(&mut self) -> Vec<ArgMatches> {
		self.arguments
			.drain(..)
			.collect()
	}

	pub fn push(&mut self, value: ArgMatches) {
		self.arguments
			.push(value);
	}
}
