use bevy;
use bevy::prelude::App;
use bevy::prelude::Plugin;
use bevy::prelude::Startup;
use bevy::prelude::Update;

mod camera;
mod light;
mod music;
mod scene;

pub struct Plugins;

impl Plugin for Plugins {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Startup,
			camera::setup,
		);

		app.add_systems(
			Update,
			camera::rotate,
		);

		app.add_systems(
			Startup,
			light::setup,
		);

		app.add_systems(
			Startup,
			music::play,
		);

		app.add_systems(
			Startup,
			scene::setup_plane,
		);

		app.add_systems(
			Update,
			scene::setup_sphere,
		);
	}
}
