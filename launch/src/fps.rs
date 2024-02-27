use bevy::{
	diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
	ecs::{
		component::Component,
		entity::Entity,
		event::{EventReader, EventWriter},
		query::With,
		schedule::{NextState, State},
		system::{Commands, Query, Res, ResMut},
	},
	hierarchy::DespawnRecursiveExt,
	render::color::Color,
	text::{Text, TextStyle},
	ui::{node_bundles::TextBundle, PositionType, Style, Val, ZIndex},
};
use clap;
use client::{Feedback, RecordLevel};

use crate::{event::FpsCommand, state::Fps};


#[derive(Component)]
pub struct FpsText;


pub fn new() -> clap::Command {
	clap::Command::new("fps")
		.about("fps display")
		.subcommand(
			clap::Command::new("open")
				.alias("1")
				.alias("on")
				.alias("display")
				.about("open the fps display, alias: 1|on|display"),
		)
		.subcommand(
			clap::Command::new("close")
				.alias("0")
				.alias("off")
				.alias("hidden")
				.about("close the fps display, alias: 0|off|hidden"),
		)
}

pub fn parse(arg: clap::ArgMatches) -> Option<FpsCommand> {
	match arg.subcommand() {
		| None => Some(FpsCommand::State),
		| Some(("open", _)) => Some(FpsCommand::Open),
		| Some(("close", _)) => Some(FpsCommand::Close),

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
	fps: Res<State<Fps>>,
	mut fps_next: ResMut<NextState<Fps>>,
	mut fps_command: EventReader<FpsCommand>,
	mut feedback: EventWriter<Feedback>,
) {
	for v in fps_command.read() {
		match v {
			| FpsCommand::Open => {
				fps_next.set(Fps::On);
			},
			| FpsCommand::Close => {
				fps_next.set(Fps::Off);
			},
			| _ => {},
		};

		let message = match v {
			| FpsCommand::Open => "fps 1",
			| FpsCommand::Close => "fps 0",
			| FpsCommand::State => {
				match fps.get() {
					| Fps::On => "fps 1",
					| Fps::Off => "fps 0",
				}
			},
		};

		feedback.send(
			Feedback::Replace(
				String::from(message),
				RecordLevel::Success,
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
			Color::rgb(
				1.0,
				0.0,
				((value - 30.0) / (60.0 - 30.0)) as f32,
			)
		}
		else if value < 60.0 {
			Color::rgb(
				1.0,
				((value - 30.0) / (60.0 - 30.0)) as f32,
				0.0,
			)
		}
		else if value < 120.0 {
			Color::rgb(
				(1.0 - (value - 60.0) / (120.0 - 60.0)) as f32,
				1.0,
				0.0,
			)
		}
		else {
			Color::rgb(
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
