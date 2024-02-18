use bevy::{
	ecs::{
		component::Component,
		entity::Entity,
		query::With,
		system::{Commands, Query},
	},
	hierarchy::DespawnRecursiveExt,
	pbr::{PointLight, PointLightBundle},
	transform::components::Transform,
};

#[derive(Component)]
pub(crate) struct Mark;

pub fn setup(mut commands: Commands) {
	commands.spawn(
		(
			Mark,
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
