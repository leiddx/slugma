use bevy::{
	app::{App, Startup},
	core_pipeline::core_3d::Camera3dBundle,
	ecs::{
		component::Component,
		entity::Entity,
		query::With,
		system::{Commands, Query, Res},
	},
	hierarchy::DespawnRecursiveExt,
	math::{Quat, Vec3},
	time::Time,
	transform::components::Transform,
};

#[derive(Component)]
pub struct Mark;

pub fn setup(mut commands: Commands) {
	commands.spawn(
		(
			Mark,
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


pub fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Mark>>) {
	let mut transform = query.single_mut();

	let rotation = Quat::from_rotation_y(time.delta_seconds() / 2.0);


	transform.rotate_around(
		Vec3::ZERO,
		rotation,
	);
}




pub struct Plugin;

impl bevy::app::Plugin for Plugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Startup, setup,
		);
	}
}
