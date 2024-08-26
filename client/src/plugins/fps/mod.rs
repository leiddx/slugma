pub mod event;
pub mod state;
pub mod ui;

use bevy::{
	app::{App, Update},
	diagnostic::FrameTimeDiagnosticsPlugin,
	ecs::schedule::{common_conditions, IntoSystemConfigs},
	state::{
		app::AppExtStates,
		condition,
		state::{OnEnter, OnExit},
	},
};

use crate::core::AddCommandEvent;


pub struct Plugin;

impl bevy::app::Plugin for Plugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(FrameTimeDiagnosticsPlugin::default());


		app.init_state::<state::Fps>();


		app.add_command_event(
			ui::new(),
			ui::parse,
		);

		app.add_systems(
			OnEnter(state::Fps::On),
			ui::setup,
		);

		app.add_systems(
			OnExit(state::Fps::On),
			ui::cleanup,
		);

		app.add_systems(
			Update,
			(
				ui::display.run_if(common_conditions::on_event::<event::Fps>()),
				ui::update.run_if(condition::in_state(state::Fps::On)),
			),
		);
	}
}
