use bevy;
use bevy::prelude::Camera;
use bevy::prelude::Camera3dBundle;
use bevy::prelude::Commands;
use bevy::prelude::Quat;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::Transform;
use bevy::prelude::Vec3;
use bevy::prelude::With;
use bevy::time::Time;

pub fn setup(mut commands: Commands) {
	commands.spawn(
		Camera3dBundle {
			transform: Transform::from_xyz(
				2.5, 5.0, 10.0,
			)
			.looking_at(
				Vec3::ZERO,
				Vec3::Y,
			),
			..bevy::prelude::default()
		},
	);
}

pub fn rotate(mut query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
	let mut transform = query.single_mut();

	transform.rotate_around(
		Vec3::ZERO,
		Quat::from_rotation_y(time.delta_seconds() / 2.0),
	);
}
