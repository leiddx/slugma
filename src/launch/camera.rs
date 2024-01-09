use bevy::{
	prelude::{Camera, Camera3dBundle, Commands, Quat, Query, Res, Transform, Vec3, With},
	time::Time,
};

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
			..Default::default()
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
