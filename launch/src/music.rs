use bevy::ecs::{
	event::{EventReader, EventWriter},
	schedule::{NextState, State},
	system::{Res, ResMut},
};
use clap;
use client::{Feedback, RecordLevel};

use crate::{event::MusicCommand, state::Music};

pub fn new() -> clap::Command {
	clap::Command::new("music")
		.about("music setting")
		.subcommand(
			clap::Command::new("stop")
				.alias("0")
				.about("stop the music"),
		)
		.subcommand(
			clap::Command::new("play")
				.alias("1")
				.about("play the music"),
		)
		.subcommand(
			clap::Command::new("pause")
				.alias("2")
				.about("pause the music"),
		)
}


pub fn parse(arg: clap::ArgMatches) -> Option<MusicCommand> {
	match arg.subcommand() {
		| None => Some(MusicCommand::State),
		| Some(("play", _)) => Some(MusicCommand::Play),
		| Some(("stop", _)) => Some(MusicCommand::Stop),
		| Some(("pause", _)) => Some(MusicCommand::Pause),

		| _ => None,
	}
}


pub fn option(
	music: Res<State<Music>>,
	mut music_next: ResMut<NextState<Music>>,
	mut music_command: EventReader<MusicCommand>,
	mut feedback: EventWriter<Feedback>,
) {
	for v in music_command.read() {
		match v {
			| MusicCommand::Stop => {
				music_next.set(Music::Stop);
			},
			| MusicCommand::Play => {
				music_next.set(Music::Play);
			},
			| MusicCommand::Pause => {
				music_next.set(Music::Pause);
			},
			| _ => {},
		};

		let message = match v {
			| MusicCommand::Stop => "music 0",
			| MusicCommand::Play => "music 1",
			| MusicCommand::Pause => "music 2",
			| MusicCommand::State => {
				match music.get() {
					| Music::Stop => "music 0",
					| Music::Play => "music 1",
					| Music::Pause => "music 2",
				}
			},
		};


		feedback.send(
			Feedback::Replace(
				String::from(message),
				RecordLevel::Success,
			),
		);
	}
}
