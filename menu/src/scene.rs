use bevy::{
	asset::Assets,
	ecs::{
		component::Component,
		entity::Entity,
		query::With,
		system::{Commands, Query, ResMut},
	},
	hierarchy::DespawnRecursiveExt,
	pbr::{PbrBundle, StandardMaterial},
	render::{
		color::Color,
		mesh::{shape::Plane, Mesh},
	},
};

#[derive(Component)]
pub(crate) struct Mark;


pub fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn(
		(
			Mark,
			PbrBundle {
				mesh: meshes.add(Mesh::from(Plane::from_size(4.0))),

				material: materials.add(
					Color::rgb(
						0.3, 0.5, 0.3,
					)
					.into(),
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
