use bevy::{app::App, DefaultPlugins};
use client;

fn main() {
	let mut app = App::new();


	app.add_plugins(DefaultPlugins);

	app.add_plugins(client::Plugins);

	app.run();
}
