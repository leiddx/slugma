use bevy::{
	asset::AssetServer,
	audio::{AudioBundle, AudioSink, AudioSinkPlayback, PlaybackMode, PlaybackSettings},
	ecs::{
		component::Component,
		entity::Entity,
		query::With,
		schedule::State,
		system::{Commands, Query, Res},
	},
	hierarchy::DespawnRecursiveExt,
};
use launch::state::Music;

#[derive(Component)]
pub(crate) struct Mark;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn(
		(
			Mark,
			AudioBundle {
				source: asset_server.load("music/windless_slopes.ogg"),

				settings: PlaybackSettings {
					paused: true,
					mode: PlaybackMode::Loop,

					..Default::default()
				},
			},
		),
	);
}

pub fn cleanup(mut commands: Commands, mark: Query<Entity, With<Mark>>) {
	for id in mark.iter() {
		commands
			.entity(id)
			.despawn_recursive();
	}
}


pub fn option(music: Res<State<Music>>, mut audio_sink: Query<&AudioSink, With<Mark>>) {
	if let Ok(sink) = audio_sink.get_single_mut() {
		match music.get() {
			| Music::Stop => {
				sink.stop();
			},
			| Music::Play => {
				sink.play();
			},
			| Music::Pause => {
				sink.pause();
			},
		};
	};
}
