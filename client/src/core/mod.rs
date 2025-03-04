mod actuator;
mod distribute;
mod history;
mod record;

pub mod event;

pub(crate) use actuator::Actuator;
use bevy::{
	app::{App, AppExit, Update},
	ecs::{
		event::{Event, EventReader, EventWriter},
		schedule::{common_conditions, IntoSystemConfigs as _},
		system::{Commands, ResMut},
	},
};
use clap::ArgMatches;
use event::{Execute, Feedback, Refresh};
pub use record::Level;




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
		if !self.is_plugin_added::<Plugin>() {
			panic!("missing plugin");
		}

		self.add_event::<T>();

		let world = self.world_mut();

		let bin = matcher
			.get_name()
			.to_string();

		let system = world.register_system(
			move |mut event: EventWriter<T>, mut actuator: ResMut<Actuator>| {
				for v in actuator.read(&bin) {
					if let Some(res) = f(v) {
						event.send(res);
					};
				}
			},
		);

		if let Some(mut actuator) = world.get_resource_mut::<Actuator>() {
			matcher = matcher.disable_help_flag(true);
			matcher = matcher.disable_help_subcommand(true);

			actuator.register(
				system, matcher,
			);
		};

		self
	}
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<Actuator>();


		app.add_event::<Execute>();
		app.add_event::<Feedback>();

		app.add_event::<Refresh>();


		app.add_systems(
			Update,
			(
				execute.run_if(common_conditions::on_event::<Execute>),
				feedback.run_if(common_conditions::on_event::<Feedback>),
			),
		);
	}
}

pub(crate) fn execute(
	mut commands: Commands,
	mut actuator: ResMut<Actuator>,
	mut execute: EventReader<Execute>,
	mut refresh: EventWriter<Refresh>,
	mut app_exit: EventWriter<AppExit>,
) {
	for v in execute.read() {
		let input = v.trim();

		actuator.push_record(
			input,
			Level::Info,
		);

		if input.eq("") {
			continue;
		}


		actuator.push_history(input);

		if let Some((bin, args)) = actuator.matcher(input) {
			if bin.eq("exit") {
				app_exit.send(AppExit::Success);

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
					Level::Success,
				);
				actuator.push_empty_record(1);

				continue;
			}

			match args.subcommand() {
				| Some(("help", _)) => {
					let help = actuator.subcommand_help(&bin);

					actuator.push_empty_record(1);
					actuator.push_record(
						&help[..],
						Level::Success,
					);
					actuator.push_empty_record(1);

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
				Level::Error,
			);
		};
	}

	refresh.send(Refresh);
}


pub(crate) fn feedback(
	mut actuator: ResMut<Actuator>,
	mut feedback: EventReader<Feedback>,
	mut refresh: EventWriter<Refresh>,
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


		refresh.send(Refresh);
	}
}
