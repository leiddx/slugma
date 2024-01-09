mod camera;
mod light;
mod music;
mod scene;

use bevy::{
	ecs::schedule::{common_conditions, IntoSystemConfigs, OnEnter},
	prelude::{App, Plugin, Update},
};

use super::state::GameState;

pub struct Plugins;

impl Plugin for Plugins {
	fn build(&self, app: &mut App) {
		app.add_state::<GameState>();

		app.add_systems(
			OnEnter(GameState::Launch),
			camera::setup,
		);

		app.add_systems(
			Update,
			camera::rotate.run_if(common_conditions::in_state(GameState::Launch)),
		);

		app.add_systems(
			OnEnter(GameState::Launch),
			light::setup,
		);

		app.add_systems(
			OnEnter(GameState::Launch),
			music::play,
		);

		app.add_systems(
			OnEnter(GameState::Launch),
			scene::setup_plane,
		);

		app.add_systems(
			Update,
			scene::setup_sphere.run_if(common_conditions::in_state(GameState::Launch)),
		);
	}
}
