use bevy::prelude::{
	shape, Assets, Color, Commands, GizmoConfig, Gizmos, Mesh, PbrBundle, Quat, ResMut,
	StandardMaterial, Vec3,
};

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
			..Default::default()
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
