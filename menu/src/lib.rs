mod camera;
mod event;
mod input;
mod light;
mod music;
mod scene;

use bevy::{
	app::{App, FixedUpdate, Plugin, Update},
	ecs::schedule::{common_conditions, IntoSystemConfigs, OnEnter},
	input::{gamepad::GamepadButtonInput, keyboard::KeyboardInput},
	time::{Fixed, Time},
};
use launch::state::{GameChapter, GameInput, Music};


pub struct Plugins;

impl Plugin for Plugins {
	fn build(&self, app: &mut App) {
		app.insert_resource(Time::<Fixed>::from_hz(2.0));

		app.add_systems(
			OnEnter(GameChapter::Menu),
			(
				camera::setup,
				light::setup,
				music::setup,
				scene::setup,
			),
		);

		app.add_systems(
			OnEnter(GameChapter::Menu),
			(
				camera::cleanup,
				light::cleanup,
				music::cleanup,
				scene::cleanup,
			),
		);

		app.add_systems(
			Update,
			(
				camera::rotate,
				input::keyboard
					.run_if(common_conditions::in_state(GameInput::Keyboard))
					.run_if(common_conditions::on_event::<KeyboardInput>()),
				input::gamepad
					.run_if(common_conditions::in_state(GameInput::Gamepad))
					.run_if(common_conditions::on_event::<GamepadButtonInput>()),
				music::option.run_if(common_conditions::state_changed::<Music>),
			)
				.run_if(common_conditions::in_state(GameChapter::Menu)),
		);

		app.add_systems(
			FixedUpdate,
			scene::display,
		);
	}
}
