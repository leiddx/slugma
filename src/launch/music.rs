use bevy::prelude::{AssetServer, AudioBundle, Commands, Res};

pub fn play(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn(
		AudioBundle {
			source: asset_server.load("music/windless_slopes.ogg"),
			..Default::default()
		},
	);
}
