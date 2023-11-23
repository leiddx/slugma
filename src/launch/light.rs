use bevy;
use bevy::prelude::Commands;
use bevy::prelude::PointLight;
use bevy::prelude::PointLightBundle;
use bevy::prelude::Transform;

pub fn setup(mut commands: Commands) {
	commands.spawn(
		PointLightBundle {
			transform: Transform::from_xyz(
				0.0, 4.8, 0.0,
			),
			point_light: PointLight {
				intensity: 3000.0,
				shadows_enabled: true,
				range: 30.0,
				..bevy::prelude::default()
			},
			..bevy::prelude::default()
		},
	);
}
