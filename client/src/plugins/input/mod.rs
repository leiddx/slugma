pub mod event;
pub mod state;


use bevy::{
	app::{App, Update},
	ecs::{
		event::{EventReader, EventWriter},
		schedule::{common_conditions, IntoSystemConfigs as _},
		system::{Res, ResMut},
	},
	state::{
		app::AppExtStates as _,
		state::{NextState, State},
	},
};

use crate::core::{event::Feedback, AddCommandEvent, Level};

pub fn new() -> clap::Command {
	clap::Command::new("input")
		.about("input type")
		.arg(
			clap::arg!([value] "0 = none; 1 = gamepad; 2 = keyboard")
				.value_parser(clap::value_parser!(u8)),
		)
}

pub fn parse(arg: clap::ArgMatches) -> Option<event::Input> {
	match arg.get_one::<u8>("value") {
		| Some(0) => Some(event::Input::None),
		| Some(1) => Some(event::Input::Gamepad),
		| Some(2) => Some(event::Input::Keyboard),

		| None => Some(event::Input::State),
		| _ => None,
	}
}

pub fn toggle(
	input: Res<State<state::Input>>,
	mut input_next: ResMut<NextState<state::Input>>,
	mut input_event: EventReader<event::Input>,
	mut feedback: EventWriter<Feedback>,
) {
	for v in input_event.read() {
		match v {
			| event::Input::None => {
				input_next.set(state::Input::None);
			},
			| event::Input::Gamepad => {
				input_next.set(state::Input::Gamepad);
			},
			| event::Input::Keyboard => {
				input_next.set(state::Input::Keyboard);
			},

			| _ => {},
		};

		let message = match v {
			| event::Input::None => state::Input::None,
			| event::Input::Gamepad => state::Input::Gamepad,
			| event::Input::Keyboard => state::Input::Keyboard,

			| event::Input::State => input.get().clone(),
		};

		feedback.send(
			Feedback::Append(
				format!(
					"input {}",
					message as u8
				),
				Level::Success,
			),
		);
	}
}


pub struct Plugin;

impl bevy::app::Plugin for Plugin {
	fn build(&self, app: &mut App) {
		app.init_state::<state::Input>();

		app.add_command_event(
			new(),
			parse,
		);


		app.add_systems(
			Update,
			(toggle.run_if(common_conditions::on_event::<event::Input>),),
		);
	}
}
