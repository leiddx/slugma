use bevy::{
	ecs::{
		event::{EventReader, EventWriter},
		schedule::{NextState, State},
		system::{Res, ResMut},
	},
	log::info,
};
use clap;
use client::{Feedback, RecordLevel};

use crate::{event::AudioCommand, state::Audio};

pub fn new() -> clap::Command {
	clap::Command::new("audio").about("audio setting, volume: 0 - 10")
}


pub fn parse(arg: clap::ArgMatches) -> Option<AudioCommand> {
	info!(
		"{:?}",
		arg
	);

	match arg.subcommand() {
		| None => Some(AudioCommand::State),

		| _ => None,
	}
}


pub fn option(
	audio: Res<State<Audio>>,
	mut audio_next: ResMut<NextState<Audio>>,
	mut audio_command: EventReader<AudioCommand>,
	mut feedback: EventWriter<Feedback>,
) {
	for v in audio_command.read() {
		match v {
			| AudioCommand::Volume(v) => {
				audio_next.set(
					if v.gt(&0) {
						Audio::Open(*v)
					}
					else {
						Audio::Close
					},
				);
			},

			| _ => {},
		};

		let message = match v {
			| AudioCommand::Volume(v) => {
				format!(
					"audio {}",
					v
				)
			},

			| AudioCommand::State => {
				match audio.get() {
					| Audio::Close => String::from("audio 0"),
					| Audio::Open(v) => {
						format!(
							"audio {}",
							v
						)
					},
				}
			},
		};


		feedback.send(
			Feedback::Replace(
				message,
				RecordLevel::Success,
			),
		);
	}
}
