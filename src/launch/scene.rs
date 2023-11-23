use bevy;
use bevy::prelude::shape;
use bevy::prelude::Assets;
use bevy::prelude::Color;
use bevy::prelude::Commands;
use bevy::prelude::GizmoConfig;
use bevy::prelude::Gizmos;
use bevy::prelude::Mesh;
use bevy::prelude::PbrBundle;
use bevy::prelude::Quat;
use bevy::prelude::ResMut;
use bevy::prelude::StandardMaterial;
use bevy::prelude::Vec3;

pub fn setup_plane(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn(
		PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Plane::from_size(4.0))),
			material: materials.add(
				Color::rgb(
					0.3, 0.5, 0.3,
				)
				.into(),
			),
			..bevy::prelude::default()
		},
	);
}

pub fn setup_sphere(mut gizmos: Gizmos, mut config: ResMut<GizmoConfig>) {
	config.line_perspective = true;

	gizmos.sphere(
		Vec3::ZERO,
		Quat::IDENTITY,
		2.0,
		Color::WHITE,
	);

	gizmos.ray(
		Vec3::new(
			-5.0, 0.0, 0.0,
		),
		Vec3::new(
			10.0, 0.0, 0.0,
		),
		Color::RED,
	);

	gizmos.ray(
		Vec3::new(
			0.0, -5.0, 0.0,
		),
		Vec3::new(
			0.0, 10.0, 0.0,
		),
		Color::BLUE,
	);

	gizmos.ray(
		Vec3::new(
			0.0, 0.0, -5.0,
		),
		Vec3::new(
			0.0, 0.0, 10.0,
		),
		Color::GREEN,
	);
}
