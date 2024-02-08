use std::{
	collections::{vec_deque::Iter, HashMap, VecDeque},
	usize,
};

use bevy::{
	app::{App, AppExit},
	ecs::{
		event::{Event, EventReader, EventWriter},
		system::{Commands, ResMut, Resource, SystemId},
	},
};
use clap::ArgMatches;
use shlex::Shlex;

use crate::{
	event::{Execute, Feedback, PanelRefresh},
	Plugins,
};


#[derive(Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum RecordLevel {
	#[default]
	Info,
	Success,
	Error,
	Warn,
	Display,
}


#[derive(Resource, Default, Debug)]
pub struct Record {
	pub max:   usize,
	pub value: VecDeque<(
		String,
		RecordLevel,
	)>,
}

impl Record {
	pub fn new(max: usize) -> Self {
		Record {
			max,
			..Default::default()
		}
	}

	pub fn clear(&mut self) {
		self.value.clear()
	}

	pub fn push(&mut self, message: &str, level: RecordLevel) {
		for v in message.split("\n") {
			self.value.push_back(
				(
					String::from(v),
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
				RecordLevel::Info,
			);
		}
	}

	pub fn replace_last(&mut self, message: &str, level: RecordLevel) {
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
		RecordLevel,
	)> {
		let last_index = self.get_last_index();

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

	pub fn get_len(&self) -> usize {
		self.value.len()
	}

	pub fn get_last_index(&self) -> usize {
		self.value
			.len()
			.saturating_sub(1)
	}
}


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


#[derive(Resource, Debug)]
struct Distribute {
	system:    SystemId,
	matcher:   clap::Command,
	arguments: Vec<ArgMatches>,
}

impl Distribute {
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

#[derive(Resource, Debug)]
pub struct Actuator {
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
	pub fn push_record(&mut self, value: &str, level: RecordLevel) {
		self.record.push(
			value, level,
		);
	}

	pub fn push_empty_record(&mut self, quantity: usize) {
		self.record
			.push_empty(quantity)
	}

	pub fn replace_last_record(&mut self, value: &str, level: RecordLevel) {
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
		RecordLevel,
	)> {
		self.record.get(
			index, quantity,
		)
	}

	pub fn get_record_len(&self) -> usize {
		self.record.get_len()
	}

	pub fn get_record_last_index(&self) -> usize {
		self.record
			.get_last_index()
	}
}

impl Actuator {
	pub fn register(&mut self, system: SystemId, matcher: clap::Command) {
		let id = String::from(matcher.get_name());
		let matcher = matcher.subcommand(clap::Command::new("help"));

		if let Some(_) = self.distribute.get(&id) {
			panic!("command does already exist");
		}

		self.matcher = self
			.matcher
			.clone()
			.subcommand(matcher.clone());

		self.distribute.insert(
			id,
			Distribute {
				system,
				matcher,
				arguments: Default::default(),
			},
		);
	}

	pub fn push(&mut self, bin: &str, args: ArgMatches) -> Result<&SystemId, ()> {
		if let Some(distribute) = self
			.distribute
			.get_mut(bin)
		{
			distribute.push(args);

			Ok(&distribute.system)
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
		&self,
		input: &str,
	) -> Option<(
		String,
		ArgMatches,
	)> {
		let matcher = self.matcher.clone();

		let mut arguments: Vec<String> = Vec::new();

		arguments.push(
			matcher
				.get_name()
				.to_string(),
		);

		for v in Shlex::new(input).collect::<Vec<_>>() {
			arguments.push(v);
		}

		if let Ok(matcher) = matcher.try_get_matches_from(arguments) {
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

	pub fn help(&self) -> String {
		let mut matcher = self.matcher.clone();

		matcher
			.render_help()
			.to_string()
	}

	pub fn subcommand_help(&self, bin: &str) -> String {
		if let Some(distribute) = self.distribute.get(bin) {
			let mut matcher = distribute
				.matcher
				.clone();

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
		let record = Record::new(500);

		let matcher = clap::Command::new("")
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


pub trait AddCommandEvent {
	fn add_command_event<T, F>(&mut self, mather: clap::Command, f: F) -> &mut Self
	where
		T: Event,
		F: Fn(ArgMatches) -> Option<T> + Send + Sync + 'static;
}

impl AddCommandEvent for App {
	fn add_command_event<T, F>(&mut self, mut matcher: clap::Command, f: F) -> &mut Self
	where
		T: Event,
		F: Fn(ArgMatches) -> Option<T> + Send + Sync + 'static,
	{
		if !self.is_plugin_added::<Plugins>() {
			panic!("missing plugin");
		}

		let bin = String::from(matcher.get_name());

		let system = self
			.world
			.register_system(
				move |mut event: EventWriter<T>, mut actuator: ResMut<Actuator>| {
					for v in actuator.read(&bin) {
						if let Some(res) = f(v) {
							event.send(res);
						};
					}
				},
			);

		self.add_event::<T>();

		if let Some(mut actuator) = self
			.world
			.get_resource_mut::<Actuator>()
		{
			matcher = matcher.disable_help_flag(true);
			matcher = matcher.disable_help_subcommand(true);

			actuator.register(
				system, matcher,
			);
		};

		self
	}
}

pub fn execute(
	mut commands: Commands,
	mut actuator: ResMut<Actuator>,
	mut execute: EventReader<Execute>,
	mut panel_refresh: EventWriter<PanelRefresh>,
	mut app_exit: EventWriter<AppExit>,
) {
	for v in execute.read() {
		let input = v.trim();

		actuator.push_record(
			input,
			RecordLevel::Info,
		);

		if input.eq("") {
			continue;
		}


		actuator.push_history(input);

		if let Some((bin, args)) = actuator.matcher(input) {
			if bin.eq("exit") {
				app_exit.send(AppExit);

				return
			}

			if bin.eq("clear") {
				actuator.clear();

				continue;
			}

			if bin.eq("help") {
				let help = actuator.help();

				actuator.push_empty_record(1);
				actuator.push_record(
					&help[..],
					RecordLevel::Success,
				);

				continue;
			}

			match args.subcommand() {
				| Some(("help", _)) => {
					let help = actuator.subcommand_help(&bin);

					actuator.push_empty_record(1);
					actuator.push_record(
						&help[..],
						RecordLevel::Success,
					);

					continue;
				},

				| _ => {},
			}

			if let Ok(id) = actuator.push(
				&bin, args,
			) {
				commands.run_system(*id);
			}
		}
		else {
			actuator.push_record(
				"command does not exist",
				RecordLevel::Error,
			);
		};
	}

	panel_refresh.send(PanelRefresh);
}


pub fn feedback(
	mut actuator: ResMut<Actuator>,
	mut feedback: EventReader<Feedback>,
	mut panel_refresh: EventWriter<PanelRefresh>,
) {
	for v in feedback.read() {
		match v {
			| Feedback::Append(message, level) => {
				actuator.push_record(
					&message,
					level.clone(),
				);
			},
			| Feedback::Replace(message, level) => {
				actuator.replace_last_record(
					&message,
					level.clone(),
				);
			},
		}


		panel_refresh.send(PanelRefresh);
	}
}
