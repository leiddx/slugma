use bevy::{
	ecs::{
		event::{EventReader, EventWriter},
		schedule::{NextState, State},
		system::{Res, ResMut},
	},
	gizmos::gizmos::Gizmos,
	math::Vec3,
	render::color::Color,
};
use client::{Feedback, RecordLevel};

use crate::{event::RulerCommand, state::Ruler};

pub fn new() -> clap::Command {
	clap::Command::new("ruler")
		.about("ruler display")
		.subcommand(
			clap::Command::new("open")
				.alias("1")
				.alias("on")
				.alias("display")
				.about("open the ruler display, alias: 1|on|display"),
		)
		.subcommand(
			clap::Command::new("close")
				.alias("0")
				.alias("off")
				.alias("hidden")
				.about("close the ruler display, alias: 0|off|hidden"),
		)
}

pub fn parse(arg: clap::ArgMatches) -> Option<RulerCommand> {
	match arg.subcommand() {
		| None => Some(RulerCommand::State),
		| Some(("open", _)) => Some(RulerCommand::Open),
		| Some(("close", _)) => Some(RulerCommand::Close),

		| _ => None,
	}
}

pub fn display(
	ruler: Res<State<Ruler>>,
	mut ruler_next: ResMut<NextState<Ruler>>,
	mut ruler_command: EventReader<RulerCommand>,
	mut feedback: EventWriter<Feedback>,
) {
	for v in ruler_command.read() {
		match v {
			| RulerCommand::Open => {
				ruler_next.set(Ruler::On);
			},
			| RulerCommand::Close => {
				ruler_next.set(Ruler::Off);
			},
			| _ => {},
		};

		let message = match v {
			| RulerCommand::Open => "ruler 1",
			| RulerCommand::Close => "ruler 0",
			| RulerCommand::State => {
				match ruler.get() {
					| Ruler::On => "ruler 1",
					| Ruler::Off => "ruler 0",
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

pub fn update(mut gizmos: Gizmos) {
	let a = -5_000.0;
	let b = 10_000.0;

	gizmos.ray(
		Vec3::new(
			a, 0.0, 0.0,
		),
		Vec3::new(
			b, 0.0, 0.0,
		),
		Color::RED,
	);

	gizmos.ray(
		Vec3::new(
			0.0, a, 0.0,
		),
		Vec3::new(
			0.0, b, 0.0,
		),
		Color::BLUE,
	);

	gizmos.ray(
		Vec3::new(
			0.0, 0.0, a,
		),
		Vec3::new(
			0.0, 0.0, b,
		),
		Color::GREEN,
	);
}
