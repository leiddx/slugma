mod console;
pub mod event;
pub mod input;
pub mod state;

use bevy::{
	app::Update,
	ecs::schedule::{common_conditions, IntoSystemConfigs, OnEnter, OnExit},
	prelude::{App, Plugin},
};

use self::{
	event::{TextInputCancelEvent, TextInputDoneEvent, TextInputEvent},
	state::ConsoleState,
};

pub struct Plugins;

impl Plugin for Plugins {
	fn build(&self, app: &mut App) {
		app.add_state::<ConsoleState>();

		app.add_event::<TextInputEvent>();
		app.add_event::<TextInputDoneEvent>();
		app.add_event::<TextInputCancelEvent>();

		app.insert_resource(console::Menu::default());
		app.insert_resource(input::TextInput::default());

		app.add_systems(
			OnEnter(ConsoleState::Open),
			console::setup,
		);

		app.add_systems(
			OnExit(ConsoleState::Open),
			console::cleanup,
		);

		app.add_systems(
			Update,
			console::listen_open.run_if(common_conditions::in_state(ConsoleState::Close)),
		);

		app.add_systems(
			Update,
			console::listen_close.run_if(common_conditions::in_state(ConsoleState::Open)),
		);

		app.add_systems(
			Update,
			console::listen_text_input
				.after(input::setup)
				.run_if(common_conditions::in_state(ConsoleState::Open)),
		);

		app.add_systems(
			Update,
			console::listen_text_input_done
				.after(input::setup)
				.run_if(common_conditions::in_state(ConsoleState::Open)),
		);

		app.add_systems(
			Update,
			console::listen_text_input_cancel
				.after(input::setup)
				.run_if(common_conditions::in_state(ConsoleState::Open)),
		);

		app.add_systems(
			Update,
			input::setup
				.after(console::listen_close)
				.run_if(common_conditions::in_state(ConsoleState::Open)),
		);

		app.add_systems(
			OnExit(ConsoleState::Open),
			input::cleanup,
		);
	}
}
