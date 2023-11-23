use bevy;
use bevy::prelude::App;
use bevy::prelude::Plugin;
use bevy::prelude::Startup;

mod console;

pub struct Plugins;

impl Plugin for Plugins {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Startup,
			console::setup,
		);
	}
}
