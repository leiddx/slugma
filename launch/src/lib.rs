pub mod audio;
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
use event::{AudioCommand, RulerCommand};

use crate::{
	event::{FpsCommand, InputCommand, MusicCommand},
	state::{Audio, Fps, GameChapter, GameInput, Music, Ruler},
};

pub struct Plugins;

impl Plugin for Plugins {
	fn build(&self, app: &mut App) {
		app.add_plugins(FrameTimeDiagnosticsPlugin::default());


		app.init_state::<Fps>();
		app.init_state::<Ruler>();
		app.init_state::<Music>();
		app.init_state::<Audio>();

		app.init_state::<GameInput>();
		app.init_state::<GameChapter>();


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

		app.add_command_event(
			audio::new(),
			audio::parse,
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
				audio::option.run_if(common_conditions::on_event::<AudioCommand>()),
			),
		);
	}
}



fn load(mut game_state: ResMut<NextState<GameChapter>>) {
	game_state.set(GameChapter::Menu);
}
