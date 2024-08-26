use bevy::{
	color::Color,
	diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
	ecs::{
		component::Component,
		entity::Entity,
		event::{EventReader, EventWriter},
		query::With,
		system::{Commands, Query, Res, ResMut},
	},
	hierarchy::DespawnRecursiveExt,
	state::state::{NextState, State},
	text::{Text, TextStyle},
	ui::{node_bundles::TextBundle, PositionType, Style, Val, ZIndex},
};
use clap;

use super::{event, state};
use crate::core::{event::Feedback, Level};



#[derive(Component)]
pub struct FpsText;


pub fn new() -> clap::Command {
	clap::Command::new("fps")
		.about("to display on the screen top-right corner")
		.arg(clap::arg!([value] "0 or 1").value_parser(clap::value_parser!(u8)))
}

pub fn parse(arg: clap::ArgMatches) -> Option<event::Fps> {
	match arg.get_one::<u8>("value") {
		| Some(0) => Some(event::Fps::Close),
		| Some(1) => Some(event::Fps::Open),

		| None => Some(event::Fps::State),
		| _ => None,
	}
}

pub fn setup(mut commands: Commands) {
	let style = Style {
		position_type: PositionType::Absolute,
		right: Val::Px(12.0),
		top: Val::Px(8.0),
		bottom: Val::Auto,
		left: Val::Auto,

		..Default::default()
	};

	let mut text = TextBundle::from_section(
		"N/A",
		TextStyle {
			font_size: 16.0,
			color: Color::WHITE,

			..Default::default()
		},
	);

	text.z_index = ZIndex::Global(i32::MAX);

	commands.spawn(
		(
			FpsText,
			text.with_style(style),
		),
	);
}

pub fn cleanup(mut commands: Commands, mark: Query<Entity, With<FpsText>>) {
	for id in mark.iter() {
		commands
			.entity(id)
			.despawn_recursive();
	}
}


pub fn display(
	fps_state: Res<State<state::Fps>>,
	mut fps_next_state: ResMut<NextState<state::Fps>>,
	mut fps_event: EventReader<event::Fps>,
	mut feedback: EventWriter<Feedback>,
) {
	for v in fps_event.read() {
		match v {
			| event::Fps::Open => {
				fps_next_state.set(state::Fps::On);
			},
			| event::Fps::Close => {
				fps_next_state.set(state::Fps::Off);
			},

			| _ => {},
		};

		let message = match v {
			| event::Fps::Open => state::Fps::On,
			| event::Fps::Close => state::Fps::Off,

			| event::Fps::State => fps_state.get().clone(),
		};

		feedback.send(
			Feedback::Append(
				format!(
					"fps {}",
					message as u8
				),
				Level::Success,
			),
		);
	}
}

pub fn update(diagnostics: Res<DiagnosticsStore>, mut fps_text: Query<&mut Text, With<FpsText>>) {
	let mut text = fps_text.single_mut();

	let (fps, color) = if let Some(value) = diagnostics
		.get(&FrameTimeDiagnosticsPlugin::FPS)
		.and_then(|fps| fps.smoothed())
	{
		let color = if value < 30.0 {
			Color::srgb(
				1.0,
				0.0,
				((value - 30.0) / (60.0 - 30.0)) as f32,
			)
		}
		else if value < 60.0 {
			Color::srgb(
				1.0,
				((value - 30.0) / (60.0 - 30.0)) as f32,
				0.0,
			)
		}
		else if value < 120.0 {
			Color::srgb(
				(1.0 - (value - 60.0) / (120.0 - 60.0)) as f32,
				1.0,
				0.0,
			)
		}
		else {
			Color::srgb(
				1.0, 0.0, 0.0,
			)
		};

		(
			format!("{value:>4.0}"),
			color,
		)
	}
	else {
		(
			String::from("N/A"),
			Color::WHITE,
		)
	};

	text.sections[0].value = fps;
	text.sections[0]
		.style
		.color = color;
}
