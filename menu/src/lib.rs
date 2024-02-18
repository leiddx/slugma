mod camera;
mod light;
mod music;
mod scene;

use bevy::{
	app::{App, Plugin, Update},
	ecs::schedule::{common_conditions, IntoSystemConfigs, OnEnter},
	time::{Fixed, Time},
};
use launch::state::{GameChapter, Music};


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
				music::option.run_if(common_conditions::state_changed::<Music>()),
			)
				.run_if(common_conditions::in_state(GameChapter::Menu)),
		);
	}
}
