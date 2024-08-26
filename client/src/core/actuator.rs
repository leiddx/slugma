use std::collections::{vec_deque::Iter, HashMap};

use bevy::{ecs::system::SystemId, prelude::Resource};
use clap::ArgMatches;
use shlex::Shlex;

use super::{
	distribute::Distribute,
	history::History,
	record::{Level, Record},
};


#[derive(Resource, Debug)]
pub(crate) struct Actuator {
	record:     Record,
	history:    History,
	distribute: HashMap<String, Distribute>,
	matcher:    clap::Command,
}

impl Actuator {
	pub fn clear(&mut self) {
		self.record.clear();
	}
}

impl Actuator {
	pub fn push_history(&mut self, value: &str) {
		self.history.push(value);
	}

	pub fn roll_back_history(&mut self) -> Option<&String> {
		self.history.roll_back()
	}

	pub fn roll_forward_history(&mut self) -> Option<&String> {
		self.history
			.roll_forward()
	}
}

impl Actuator {
	pub fn push_record(&mut self, value: &str, level: Level) {
		self.record.push(
			value, level,
		);
	}

	pub fn push_empty_record(&mut self, quantity: usize) {
		self.record
			.push_empty(quantity)
	}

	pub fn replace_last_record(&mut self, value: &str, level: Level) {
		self.record
			.replace_last(
				value, level,
			);
	}

	pub fn get_record(
		&self,
		index: &Option<usize>,
		quantity: &usize,
	) -> Iter<(
		String,
		Level,
	)> {
		self.record.get(
			index, quantity,
		)
	}

	pub fn record_len(&self) -> usize {
		self.record.len()
	}

	pub fn record_last_index(&self) -> usize {
		self.record.last_index()
	}
}

impl Actuator {
	pub fn register(&mut self, system: SystemId, matcher: clap::Command) {
		let id = matcher
			.get_name()
			.to_string();

		let matcher = matcher.subcommand(clap::Command::new("help"));

		if let Some(_) = self.distribute.get(&id) {
			panic!("command does already exist");
		}

		self.matcher = self
			.matcher
			.clone()
			.subcommand(matcher);

		self.distribute.insert(
			id,
			Distribute::new(system),
		);
	}

	pub fn push(&mut self, bin: &str, args: ArgMatches) -> Result<&SystemId, ()> {
		if let Some(distribute) = self
			.distribute
			.get_mut(bin)
		{
			distribute.push(args);

			Ok(distribute.system_id())
		}
		else {
			Err(())
		}
	}

	pub fn read(&mut self, bin: &str) -> Vec<ArgMatches> {
		if let Some(distribute) = self
			.distribute
			.get_mut(bin)
		{
			distribute.read()
		}
		else {
			vec![]
		}
	}

	pub fn matcher(
		&mut self,
		input: &str,
	) -> Option<(
		String,
		ArgMatches,
	)> {
		let arguments = Shlex::new(input).collect::<Vec<_>>();


		if let Ok(matcher) = self
			.matcher
			.try_get_matches_from_mut(arguments)
		{
			if let Some((bin, args)) = matcher.subcommand() {
				return Some(
					(
						bin.to_string(),
						args.clone(),
					),
				)
			}
		}

		None
	}

	pub fn help(&mut self) -> String {
		self.matcher
			.render_help()
			.to_string()
	}

	pub fn subcommand_help(&mut self, bin: &str) -> String {
		if let Some(matcher) = self
			.matcher
			.find_subcommand_mut(bin)
		{
			matcher
				.render_help()
				.to_string()
		}
		else {
			String::from("")
		}
	}
}

impl Default for Actuator {
	fn default() -> Self {
		let record = Record::new(1024);

		let matcher = clap::Command::new("")
			.no_binary_name(true)
			.disable_help_flag(true)
			.disable_help_subcommand(true)
			.subcommand(clap::Command::new("help").about("show the commands"))
			.subcommand(
				clap::Command::new("exit")
					.alias("quit")
					.about("exit game"),
			)
			.subcommand(
				clap::Command::new("clear")
					.alias("clean")
					.about("clear the record"),
			);

		Self {
			record,
			history: Default::default(),
			distribute: Default::default(),
			matcher,
		}
	}
}
