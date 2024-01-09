mod game;
mod launch;
mod state;

use bevy::{app::App, DefaultPlugins};

fn main() {
	let mut app = App::new();

	app.add_plugins(DefaultPlugins);

	app.add_plugins(game::Plugins);
	app.add_plugins(launch::Plugins);

	app.run();
}
