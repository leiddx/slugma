use bevy::{
	color::Color,
	ecs::{
		component::Component,
		event::{EventReader, EventWriter},
		query::With,
		system::{Query, Res, ResMut},
	},
	input::{
		keyboard::KeyCode,
		mouse::{MouseScrollUnit, MouseWheel},
		ButtonInput,
	},
	text::{TextColor, TextFont},
	ui::{widget::Text, Node},
	window::WindowResized,
};

use super::event::PanelRefresh;
use crate::core::{event::Refresh, Actuator, Level};


#[derive(Component)]
pub struct Label;


#[derive(Component, Default)]
pub(crate) struct Panel {
	pub style:       TextFont,
	pub last_index:  Option<usize>,
	pub scroll_step: usize,
	pub max_display: usize,
}

impl Panel {
	pub(crate) fn new(height: f32, style: TextFont) -> Self {
		let mut panel = Self {
			style,

			..Default::default()
		};

		panel.limit_display(height);

		panel
	}

	pub fn bubble(&mut self) {
		self.last_index = None;
	}

	pub fn limit_display(&mut self, height: f32) {
		let display = (height - 30.0 - self.style.font_size) / self.style.font_size;

		let max_display = display as usize;

		self.max_display = max_display;
		self.scroll_step = max_display / 10;
	}

	pub fn dyeing(&self, level: &Level) -> Color {
		match level {
			| Level::Info => Color::WHITE,
			| Level::Success => {
				Color::srgb_u8(
					0, 255, 0,
				)
			},
			| Level::Error => {
				Color::srgb_u8(
					255, 0, 0,
				)
			},
			| Level::Warn => {
				Color::srgb_u8(
					255, 165, 0,
				)
			},
			| Level::Display => {
				Color::srgba_u8(
					255, 255, 255, 128,
				)
			},
		}
	}

	pub fn from_section(
		&self,
		text: &str,
	) -> (
		Text,
		TextFont,
		TextColor,
		Node,
	) {
		(
			Text::new(text),
			TextFont {
				..self.style.clone()
			},
			TextColor::from(self.dyeing(&Level::Display)),
			Node {
				..Default::default()
			},
		)
	}
}



pub fn scroll_up(
	actuator: Res<Actuator>,
	mut key_code: ResMut<ButtonInput<KeyCode>>,
	mut panel: Query<&mut Panel>,
	mut panel_refresh: EventWriter<PanelRefresh>,
) {
	if key_code.just_pressed(KeyCode::PageUp) {
		let mut panel = panel.single_mut();

		let last_index = panel
			.last_index
			.unwrap_or_else(|| actuator.record_last_index());

		panel.last_index = Some(last_index.saturating_sub(panel.scroll_step));

		panel_refresh.send(PanelRefresh);

		key_code.clear();
	}
}


pub fn scroll_down(
	mut key_code: ResMut<ButtonInput<KeyCode>>,
	mut panel: Query<&mut Panel>,
	mut panel_refresh: EventWriter<PanelRefresh>,
) {
	if key_code.just_pressed(KeyCode::PageDown) {
		let mut panel = panel.single_mut();

		panel.last_index = if let Some(v) = panel.last_index {
			Some(v.saturating_add(panel.scroll_step))
		}
		else {
			None
		};

		panel_refresh.send(PanelRefresh);

		key_code.clear();
	}
}


pub fn scroll_wheel(
	actuator: Res<Actuator>,
	mut mouse_wheel: EventReader<MouseWheel>,
	mut panel: Query<&mut Panel>,
	mut panel_refresh: EventWriter<PanelRefresh>,
) {
	let mut panel = panel.single_mut();

	let mut last_index = if let Some(v) = panel.last_index {
		v
	}
	else {
		actuator.record_last_index()
	};

	for v in mouse_wheel.read() {
		match v.unit {
			| MouseScrollUnit::Line => {
				if v.y > 0.0 {
					last_index = last_index.saturating_sub(panel.scroll_step);
				}

				if v.y < 0.0 {
					last_index = last_index.saturating_add(panel.scroll_step);
				}
			},
			| _ => (),
		}
	}

	panel.last_index = Some(last_index);

	panel_refresh.send(PanelRefresh);
}

pub fn resize(
	mut panel: Query<&mut Panel>,
	mut window_resized: EventReader<WindowResized>,
	mut panel_refresh: EventWriter<PanelRefresh>,
) {
	let mut panel = panel.single_mut();

	for v in window_resized.read() {
		panel.limit_display(v.height);
	}

	panel_refresh.send(PanelRefresh);
}

pub fn refresh(
	actuator: Res<Actuator>,
	mut panel: Query<&mut Panel>,
	mut label: Query<
		(
			&mut Text,
			&mut TextColor,
		),
		With<Label>,
	>,
	mut panel_refresh: EventReader<PanelRefresh>,
) {
	panel_refresh.clear();

	let mut panel = panel.single_mut();

	if panel.max_display >= actuator.record_len() {
		panel.last_index = None;
	}

	if let Some(v) = panel.last_index {
		panel.last_index = if actuator.record_last_index() > v {
			let first_index = panel
				.max_display
				.saturating_sub(1);

			Some(v.max(first_index))
		}
		else {
			None
		}
	}

	let item: Vec<_> = actuator
		.get_record(
			&panel.last_index, &panel.max_display,
		)
		.collect();

	let start = panel
		.max_display
		.saturating_sub(item.len());

	for (i, mut v) in label
		.iter_mut()
		.enumerate()
	{
		let (text, level) = if i < start {
			(
				String::from(""),
				Level::Display,
			)
		}
		else {
			let (text, level) = item[i.saturating_sub(start)];

			(
				text.clone(),
				level.clone(),
			)
		};

		**v.0 = text;
		**v.1 = panel.dyeing(&level);
	}
}




pub fn refresh_transfer(
	mut actuator_refresh: EventReader<Refresh>,
	mut panel_refresh: EventWriter<PanelRefresh>,
) {
	for _ in actuator_refresh.read() {
		panel_refresh.send(PanelRefresh);
	}
}
