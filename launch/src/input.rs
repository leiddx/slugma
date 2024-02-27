use bevy::ecs::{
	event::{EventReader, EventWriter},
	schedule::{NextState, State},
	system::{Res, ResMut},
};
use client::{Feedback, RecordLevel};

use crate::{event::InputCommand, state::GameInput};



pub fn new() -> clap::Command {
	clap::Command::new("input")
		.about("input type")
		.subcommand(
			clap::Command::new("none")
				.alias("0")
				.about("toggle input to none"),
		)
		.subcommand(
			clap::Command::new("gamepad")
				.alias("1")
				.about("toggle input to gamepad"),
		)
		.subcommand(
			clap::Command::new("keyboard")
				.alias("2")
				.about("toggle input to keyboard"),
		)
}

pub fn parse(arg: clap::ArgMatches) -> Option<InputCommand> {
	match arg.subcommand() {
		| None => Some(InputCommand::State),
		| Some(("none", _)) => Some(InputCommand::None),
		| Some(("gamepad", _)) => Some(InputCommand::Gamepad),
		| Some(("keyboard", _)) => Some(InputCommand::Keyboard),

		| _ => None,
	}
}


pub fn toggle(
	game_input: Res<State<GameInput>>,
	mut game_input_next: ResMut<NextState<GameInput>>,
	mut game_input_command: EventReader<InputCommand>,
	mut feedback: EventWriter<Feedback>,
) {
	for v in game_input_command.read() {
		match v {
			| InputCommand::None => {
				game_input_next.set(GameInput::None);
			},
			| InputCommand::Gamepad => {
				game_input_next.set(GameInput::Gamepad);
			},
			| InputCommand::Keyboard => {
				game_input_next.set(GameInput::Keyboard);
			},

			| _ => {},
		};

		let message = match v {
			| InputCommand::None => "input 0",
			| InputCommand::Gamepad => "input 1",
			| InputCommand::Keyboard => "input 2",
			| InputCommand::State => {
				match game_input.get() {
					| GameInput::None => "input 0",
					| GameInput::Gamepad => "input 1",
					| GameInput::Keyboard => "input 2",
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
