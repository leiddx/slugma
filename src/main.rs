use bevy::prelude::App;
use bevy::DefaultPlugins;

mod game;
mod launch;

fn main() {
	let mut app = App::new();

	app.add_plugins(DefaultPlugins);

	app.add_plugins(game::Plugins);
	app.add_plugins(launch::Plugins);

	app.run();
}
