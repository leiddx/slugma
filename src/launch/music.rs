use bevy;
use bevy::prelude::AssetServer;
use bevy::prelude::AudioBundle;
use bevy::prelude::Commands;
use bevy::prelude::Res;

pub fn play(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn(
		AudioBundle {
			source: asset_server.load("music/windless_slopes.ogg"),
			..bevy::prelude::default()
		},
	);
}
