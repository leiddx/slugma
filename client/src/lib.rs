mod command;
mod console;
mod event;
mod state;
mod window;

pub mod character;

use bevy::{
	app::{App, Plugin, Startup, Update},
	ecs::{
		schedule::{
			common_conditions, Condition, IntoSystemConfigs, IntoSystemSetConfigs, OnEnter, OnExit,
			SystemSet,
		},
		system::IntoSystem,
	},
	input::{keyboard::KeyboardInput, mouse::MouseWheel},
	window::{ReceivedCharacter, WindowFocused, WindowResized},
};

use crate::{
	command::Actuator,
	event::{PanelRefresh, PromptRefresh, UpdateCharacter},
	state::Console,
};
pub use crate::{
	command::{AddCommandEvent, RecordLevel},
	event::{Execute, Feedback},
};


#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum Step {
	Receive,
	Update,
	Refresh,
}


pub struct Plugins;

impl Plugin for Plugins {
	fn build(&self, app: &mut App) {
		app.init_state::<Console>();

		app.init_resource::<Actuator>();

		app.add_event::<PanelRefresh>();
		app.add_event::<PromptRefresh>();

		app.add_event::<UpdateCharacter>();

		app.add_event::<Execute>();
		app.add_event::<Feedback>();


		app.configure_sets(
			Update,
			(
				Step::Receive,
				Step::Update,
				Step::Refresh,
			)
				.chain()
				.run_if(common_conditions::in_state(Console::Open)),
		);

		app.add_systems(
			Startup,
			window::setup,
		);

		app.add_systems(
			OnEnter(Console::Open),
			(
				console::setup,
				character::setup,
			),
		);

		app.add_systems(
			OnExit(Console::Open),
			(
				console::cleanup,
				character::cleanup,
			),
		);

		app.add_systems(
			Update,
			(
				window::focus.run_if(common_conditions::on_event::<WindowFocused>()),
				window::change_mode.run_if(common_conditions::on_event::<KeyboardInput>()),
				command::execute.run_if(common_conditions::on_event::<Execute>()),
				command::feedback.run_if(common_conditions::on_event::<Feedback>()),
			),
		);

		app.add_systems(
			Update,
			console::open
				.run_if(common_conditions::on_event::<KeyboardInput>())
				.run_if(common_conditions::in_state(Console::Close)),
		);

		app.add_systems(
			Update,
			character::received
				.pipe(console::received_character)
				.before(command::execute)
				.run_if(
					common_conditions::on_event::<KeyboardInput>()
						.or_else(common_conditions::on_event::<UpdateCharacter>())
						.or_else(common_conditions::on_event::<ReceivedCharacter>()),
				)
				.in_set(Step::Receive),
		);

		app.add_systems(
			Update,
			(
				(
					console::close,
					console::history_roll_back,
					console::history_roll_forward,
					console::panel_scroll_up,
					console::panel_scroll_down,
				)
					.run_if(common_conditions::on_event::<KeyboardInput>()),
				console::panel_scroll_wheel.run_if(common_conditions::on_event::<MouseWheel>()),
				console::panel_resize.run_if(common_conditions::on_event::<WindowResized>()),
			)
				.in_set(Step::Update),
		);

		app.add_systems(
			Update,
			(
				console::panel_refresh
					.after(command::execute)
					.run_if(common_conditions::on_event::<PanelRefresh>()),
				console::prompt_refresh.run_if(common_conditions::on_event::<PromptRefresh>()),
			)
				.in_set(Step::Refresh),
		);
	}
}
