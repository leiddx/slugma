pub mod event;
pub mod fps;
pub mod input;
pub mod music;
pub mod ruler;
pub mod state;

use bevy::{
	app::{App, Plugin, Startup, Update},
	diagnostic::FrameTimeDiagnosticsPlugin,
	ecs::{
		schedule::{common_conditions, IntoSystemConfigs, NextState, OnEnter, OnExit},
		system::ResMut,
	},
};
use client::AddCommandEvent;
use event::RulerCommand;

use crate::{
	event::{FpsCommand, InputCommand, MusicCommand},
	state::{Fps, GameChapter, GameInput, Music, Ruler},
};

pub struct Plugins;

impl Plugin for Plugins {
	fn build(&self, app: &mut App) {
		app.add_plugins(FrameTimeDiagnosticsPlugin::default());


		app.add_state::<Fps>();
		app.add_state::<Ruler>();
		app.add_state::<Music>();

		app.add_state::<GameInput>();
		app.add_state::<GameChapter>();


		app.add_command_event(
			fps::new(),
			fps::parse,
		);

		app.add_command_event(
			ruler::new(),
			ruler::parse,
		);

		app.add_command_event(
			input::new(),
			input::parse,
		);

		app.add_command_event(
			music::new(),
			music::parse,
		);


		app.add_systems(
			Startup, load,
		);

		app.add_systems(
			OnEnter(Fps::On),
			fps::setup,
		);

		app.add_systems(
			OnExit(Fps::On),
			fps::cleanup,
		);

		app.add_systems(
			Update,
			(
				fps::display.run_if(common_conditions::on_event::<FpsCommand>()),
				fps::update.run_if(common_conditions::in_state(Fps::On)),
				ruler::display.run_if(common_conditions::on_event::<RulerCommand>()),
				ruler::update.run_if(common_conditions::in_state(Ruler::On)),
				input::toggle.run_if(common_conditions::on_event::<InputCommand>()),
				music::option.run_if(common_conditions::on_event::<MusicCommand>()),
			),
		);
	}
}



fn load(mut game_state: ResMut<NextState<GameChapter>>) {
	game_state.set(GameChapter::Menu);
}
