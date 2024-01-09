use bevy::prelude::{Commands, PointLight, PointLightBundle, Transform};

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
				..Default::default()
			},
			..Default::default()
		},
	);
}
