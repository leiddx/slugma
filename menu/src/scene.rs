use bevy::{
	asset::Assets,
	ecs::{
		component::Component,
		entity::Entity,
		event::EventWriter,
		query::With,
		system::{Commands, Query, ResMut},
	},
	hierarchy::DespawnRecursiveExt,
	math::primitives::Plane3d,
	pbr::{PbrBundle, StandardMaterial},
	render::{
		color::Color,
		mesh::{Mesh, Meshable},
	},
};
use client::{Feedback, RecordLevel};

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
				mesh: meshes.add(
					Plane3d::default()
						.mesh()
						.size(
							5.0, 5.0,
						),
				),

				material: materials.add(
					Color::rgb(
						0.3, 0.5, 0.3,
					),
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


pub fn display(mut feedback: EventWriter<Feedback>) {
	feedback.send(
		Feedback::Append(
			String::from("大风云起"),
			RecordLevel::Display,
		),
	);
}
