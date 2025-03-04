mod event;
mod history;
mod panel;
mod prompt;
mod ui;

pub mod state;

use bevy::{
	app::{App, Update},
	ecs::{
		schedule::{
			common_conditions, Condition as _, IntoSystemConfigs as _, IntoSystemSetConfigs as _,
			SystemSet,
		},
		system::IntoSystem as _,
	},
	input::{keyboard::KeyboardInput, mouse::MouseWheel},
	state::{
		app::AppExtStates as _,
		condition,
		state::{OnEnter, OnExit},
	},
	window::WindowResized,
};
use event::{PanelRefresh, PromptRefresh};
use state::Console;

use super::character;
use crate::core::{self, event::Refresh};



#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum Step {
	Receive,
	Update,
	Refresh,
}



pub struct Plugin;

impl bevy::app::Plugin for Plugin {
	fn build(&self, app: &mut App) {
		app.init_state::<Console>();


		app.add_event::<PanelRefresh>();
		app.add_event::<PromptRefresh>();

		app.add_event::<character::event::Update>();


		app.configure_sets(
			Update,
			(
				Step::Receive,
				Step::Update,
				Step::Refresh,
			)
				.chain()
				.run_if(condition::in_state(Console::Open)),
		);

		app.add_systems(
			OnEnter(Console::Open),
			(
				ui::setup,
				character::setup,
			),
		);

		app.add_systems(
			OnExit(Console::Open),
			(
				ui::cleanup,
				character::cleanup,
			),
		);


		app.add_systems(
			Update,
			ui::open
				.run_if(common_conditions::on_event::<KeyboardInput>)
				.run_if(condition::in_state(Console::Close)),
		);

		app.add_systems(
			Update,
			character::received
				.pipe(ui::received_character)
				.before(core::execute)
				.run_if(
					common_conditions::on_event::<KeyboardInput>
						.or(common_conditions::on_event::<character::event::Update>),
				)
				.in_set(Step::Receive),
		);

		app.add_systems(
			Update,
			(
				(
					ui::close,
					history::roll_back,
					history::roll_forward,
					panel::scroll_up,
					panel::scroll_down,
				)
					.run_if(common_conditions::on_event::<KeyboardInput>),
				panel::scroll_wheel.run_if(common_conditions::on_event::<MouseWheel>),
				panel::resize.run_if(common_conditions::on_event::<WindowResized>),
			)
				.in_set(Step::Update),
		);

		app.add_systems(
			Update,
			(
				panel::refresh_transfer
					.after(core::execute)
					.run_if(common_conditions::on_event::<Refresh>),
				panel::refresh
					.after(panel::refresh_transfer)
					.run_if(common_conditions::on_event::<PanelRefresh>),
				prompt::refresh.run_if(common_conditions::on_event::<PromptRefresh>),
			)
				.in_set(Step::Refresh),
		);
	}
}
