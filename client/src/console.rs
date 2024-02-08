use bevy::{
	asset::AssetServer,
	ecs::{
		component::Component,
		entity::Entity,
		event::{EventReader, EventWriter},
		query::With,
		schedule::NextState,
		system::{Commands, In, Query, Res, ResMut},
	},
	hierarchy::{BuildChildren, DespawnRecursiveExt},
	input::{
		keyboard::KeyCode,
		mouse::{MouseScrollUnit, MouseWheel},
		Input,
	},
	render::color::Color,
	text::{Text, TextStyle},
	ui::{
		node_bundles::{NodeBundle, TextBundle},
		AlignItems, BackgroundColor, Display, FlexDirection, JustifyContent, Overflow, Style,
		UiRect, Val,
	},
	window::{Window, WindowResized},
};

use crate::{
	command::{Actuator, RecordLevel},
	event::{Execute, PanelRefresh, PromptRefresh, UpdateCharacter},
	state::Console,
};


#[derive(Component)]
pub struct Label;

#[derive(Component)]
pub struct Prompt;


#[derive(Component, Default)]
pub(crate) struct Panel {
	pub style:       TextStyle,
	pub last_index:  Option<usize>,
	pub scroll_step: usize,
	pub max_display: usize,
}

impl Panel {
	fn new(height: &f32, style: TextStyle) -> Self {
		let mut panel = Panel {
			style,

			..Default::default()
		};

		panel.limit_display(height);

		panel
	}

	pub fn bubble(&mut self) {
		self.last_index = None;
	}

	pub fn limit_display(&mut self, height: &f32) {
		let height = height - 42.0;

		let max_display = height / self.style.font_size;

		self.max_display = max_display as usize;
		self.scroll_step = (max_display / 10.0).ceil() as usize;
	}

	pub fn parse(
		&self,
		text: &str,
		level: &RecordLevel,
	) -> (
		String,
		Color,
	) {
		let text = format!(
			"  {}",
			text
		);

		let color = match level {
			| RecordLevel::Info => Color::WHITE,
			| RecordLevel::Success => Color::GREEN,
			| RecordLevel::Error => Color::RED,
			| RecordLevel::Warn => Color::ORANGE,
			| RecordLevel::Display => Color::GRAY,
		};

		(
			text, color,
		)
	}

	pub fn format(&self, text: &str, level: &RecordLevel) -> TextBundle {
		let (text, color) = self.parse(
			text, level,
		);

		TextBundle::from_section(
			text,
			TextStyle {
				color,
				..self.style.clone()
			},
		)
	}
}


pub fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut window: Query<&mut Window>,
	mut panel_refresh: EventWriter<PanelRefresh>,
) {
	let window = window.single_mut();

	let style = TextStyle {
		font: asset_server.load("fonts/FiraMono-Medium.ttf"),
		font_size: 14.0,

		..Default::default()
	};


	let panel = Panel::new(
		&window.height(),
		style.clone(),
	);

	let mut label: Vec<Entity> = Vec::new();

	for _ in 0..panel.max_display {
		let id = commands
			.spawn(
				(
					Label,
					panel.format(
						"",
						&RecordLevel::Display,
					),
				),
			)
			.id();

		label.push(id);
	}

	let root = commands
		.spawn(
			(
				panel,
				NodeBundle {
					style: Style {
						display: Display::Flex,
						flex_direction: FlexDirection::Column,
						justify_content: JustifyContent::FlexEnd,
						width: Val::Vw(100.0),
						height: Val::Vh(100.0),
						padding: UiRect::all(Val::Px(10.0)),
						border: UiRect::all(Val::Px(10.0)),
						overflow: Overflow::clip(),

						..Default::default()
					},

					background_color: BackgroundColor(
						Color::rgba(
							0.0, 0.0, 0.0, 0.75,
						),
					),

					..Default::default()
				},
			),
		)
		.id();

	let prompt = commands
		.spawn(
			NodeBundle {
				style: Style {
					display: Display::Flex,
					align_items: AlignItems::Center,
					width: Val::Percent(100.0),
					margin: UiRect::top(Val::Px(8.0)),

					..Default::default()
				},

				..Default::default()
			},
		)
		.with_children(
			|parent| {
				parent.spawn(
					(
						Prompt,
						TextBundle::from_section(
							"> _",
							style.clone(),
						),
					),
				);
			},
		)
		.id();

	commands
		.entity(root)
		.add_child(prompt)
		.insert_children(
			0, &label,
		);

	panel_refresh.send(PanelRefresh);
}

pub fn cleanup(mut commands: Commands, panel: Query<Entity, With<Panel>>) {
	for id in panel.iter() {
		commands
			.entity(id)
			.despawn_recursive();
	}
}

pub fn open(mut key_code: ResMut<Input<KeyCode>>, mut console: ResMut<NextState<Console>>) {
	if key_code.just_pressed(KeyCode::Grave) {
		console.set(Console::Open);

		key_code.clear();
	}
}

pub fn close(
	mut key_code: ResMut<Input<KeyCode>>,
	mut console: ResMut<NextState<Console>>,
	mut prompt_refresh: EventWriter<PromptRefresh>,
) {
	if key_code.any_just_pressed([KeyCode::Grave, KeyCode::Escape]) {
		console.set(Console::Close);

		key_code.clear();
		prompt_refresh.send(PromptRefresh(String::from("")));
	}
}

pub fn received_character(
	In((character, enter)): In<(
		String,
		bool,
	)>,
	mut panel: Query<&mut Panel>,
	mut execute: EventWriter<Execute>,
	mut prompt_refresh: EventWriter<PromptRefresh>,
) {
	let mut panel = panel.single_mut();

	if enter {
		panel.bubble();

		execute.send(Execute::new(character));
		prompt_refresh.send(PromptRefresh(String::from("")));
	}
	else {
		prompt_refresh.send(PromptRefresh(character));
	}
}


pub fn history_roll_back(
	mut actuator: ResMut<Actuator>,
	mut key_code: ResMut<Input<KeyCode>>,
	mut prompt_refresh: EventWriter<PromptRefresh>,
	mut update_character: EventWriter<UpdateCharacter>,
) {
	if key_code.just_pressed(KeyCode::Up) {
		key_code.clear();

		if let Some(message) = actuator.roll_back_history() {
			prompt_refresh.send(PromptRefresh(message.clone()));
			update_character.send(UpdateCharacter(message.clone()));
		}
	}
}

pub fn history_roll_forward(
	mut actuator: ResMut<Actuator>,
	mut key_code: ResMut<Input<KeyCode>>,
	mut prompt_refresh: EventWriter<PromptRefresh>,
	mut update_character: EventWriter<UpdateCharacter>,
) {
	if key_code.just_pressed(KeyCode::Down) {
		let message = if let Some(message) = actuator.roll_forward_history() {
			message.clone()
		}
		else {
			String::from("")
		};

		key_code.clear();

		prompt_refresh.send(PromptRefresh(message.clone()));
		update_character.send(UpdateCharacter(message.clone()));
	}
}


pub fn panel_scroll_up(
	actuator: Res<Actuator>,
	mut key_code: ResMut<Input<KeyCode>>,
	mut panel: Query<&mut Panel>,
	mut panel_refresh: EventWriter<PanelRefresh>,
) {
	if key_code.just_pressed(KeyCode::PageUp) {
		let mut panel = panel.single_mut();

		let last_index = panel
			.last_index
			.unwrap_or_else(|| actuator.get_record_last_index());

		panel.last_index = Some(last_index.saturating_sub(panel.scroll_step));

		panel_refresh.send(PanelRefresh);

		key_code.clear();
	}
}


pub fn panel_scroll_down(
	mut key_code: ResMut<Input<KeyCode>>,
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


pub fn panel_scroll_wheel(
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
		actuator.get_record_last_index()
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

pub fn panel_resize(
	mut panel: Query<&mut Panel>,
	mut window_resized: EventReader<WindowResized>,
	mut panel_refresh: EventWriter<PanelRefresh>,
) {
	let mut panel = panel.single_mut();

	for v in window_resized.read() {
		panel.limit_display(&v.height);
	}

	panel_refresh.send(PanelRefresh);
}

pub fn panel_refresh(
	actuator: Res<Actuator>,
	mut panel: Query<&mut Panel>,
	mut label: Query<&mut Text, With<Label>>,
	mut panel_refresh: EventReader<PanelRefresh>,
) {
	panel_refresh.clear();

	let mut panel = panel.single_mut();

	if panel.max_display >= actuator.get_record_len() {
		panel.last_index = None;
	}

	if let Some(v) = panel.last_index {
		panel.last_index = if actuator.get_record_last_index() > v {
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

	let padding = panel
		.max_display
		.saturating_sub(item.len());

	for (i, mut v) in label
		.iter_mut()
		.enumerate()
	{
		let (text, level) = if i < padding {
			(
				String::from(""),
				RecordLevel::Display,
			)
		}
		else {
			let (text, level) = item[i.saturating_sub(padding)];

			(
				text.clone(),
				level.clone(),
			)
		};

		let (text, color) = panel.parse(
			&text, &level,
		);

		v.sections[0].value = text;
		v.sections[0]
			.style
			.color = color;
	}
}


pub fn prompt_refresh(
	mut prompt: Query<&mut Text, With<Prompt>>,
	mut prompt_refresh: EventReader<PromptRefresh>,
) {
	let mut text = prompt.single_mut();

	for v in prompt_refresh.read() {
		text.sections[0].value = format!(
			"> {}_",
			v.0.to_string()
		);
	}
}
