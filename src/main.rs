use bevy::{app::App, DefaultPlugins};
use client;
use launch;
use menu;

fn main() {
	let mut app = App::new();


	app.add_plugins(DefaultPlugins);

	app.add_plugins(client::Plugins);
	app.add_plugins(launch::Plugins);
	app.add_plugins(menu::Plugins);

	app.run();
}
