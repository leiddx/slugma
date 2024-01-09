use bevy::{
	a11y::{
		accesskit::{NodeBuilder, Role},
		AccessibilityNode,
	},
	app::AppExit,
	asset::AssetServer,
	ecs::{
		component::Component,
		entity::Entity,
		event::{EventReader, EventWriter},
		schedule::NextState,
		system::{Commands, Query, Res, ResMut, Resource},
	},
	hierarchy::{BuildChildren, DespawnRecursiveExt},
	input::{keyboard::KeyCode, Input},
	render::color::Color,
	text::{Text, TextStyle},
	ui::{
		node_bundles::{NodeBundle, TextBundle},
		widget::Label,
		AlignItems, BackgroundColor, Display, FlexDirection, Overflow, Style, UiRect, Val,
	},
};

use super::{
	event::{TextInputCancelEvent, TextInputDoneEvent, TextInputEvent},
	state::ConsoleState,
};

#[derive(Resource, Default)]
pub struct Menu {
	id: Option<Entity>,
	history: MenuHistory,
	input: MenuInput,
}

impl Menu {
	pub fn empty(&mut self) -> &Self {
		self.id = None;
		self.history.empty();
		self.input.empty();

		self
	}

	pub fn save(&mut self, id: Entity, input: Entity, history: Entity) -> &Self {
		self.id = Some(id);
		self.input.id = Some(input);
		self.history.id = Some(history);

		self
	}
}

#[derive(Component, Default)]
struct MenuHistory {
	id: Option<Entity>,
	value: Vec<String>,
	position: f32,
}

impl MenuHistory {
	pub fn push(&mut self, value: &String) -> &Self {
		self.value
			.push(value.to_string());

		self
	}

	pub fn empty(&mut self) -> &Self {
		self.id = None;

		self
	}

	pub fn clear(&mut self, commands: &mut Commands) -> &Self {
		if let Some(id) = self.id {
			self.value.clear();

			commands
				.entity(id)
				.despawn_descendants();
		}

		self
	}

	pub fn show(&self, message: &String, style: &TextStyle, commands: &mut Commands) -> &Self {
		if let Some(id) = self.id {
			let text = commands
				.spawn(
					(
						TextBundle::from_section(
							message.clone(),
							style.clone(),
						),
						Label,
						AccessibilityNode(NodeBuilder::new(Role::ListItem)),
					),
				)
				.id();

			commands
				.entity(id)
				.add_child(text);
		}

		self
	}

	pub fn pshow(&mut self, message: &String, style: &TextStyle, commands: &mut Commands) -> &Self {
		self.push(message);
		self.show(
			message, style, commands,
		);

		self
	}

	pub fn restore(&self, style: &TextStyle, commands: &mut Commands) -> &Self {
		for v in &self.value {
			self.show(
				v, style, commands,
			);
		}

		self
	}
}

#[derive(Component, Default)]
struct MenuInput {
	id: Option<Entity>,
	value: String,
}

impl MenuInput {
	pub fn is_empty(&self) -> bool {
		self.value.is_empty()
	}

	pub fn to_string(&self) -> String {
		self.value.to_string()
	}

	pub fn set(&mut self, value: &String) -> &Self {
		self.value = value.to_string();

		self
	}

	pub fn empty(&mut self) -> &Self {
		self.id = None;
		self.value.clear();

		self
	}

	pub fn clear(&mut self) -> &Self {
		self.value.clear();

		self
	}

	pub fn show(&self, mut query: Query<&mut Text>) -> &Self {
		if let Some(id) = self.id {
			if let Ok(mut text) = query.get_mut(id) {
				text.sections[0].value = self.to_string();
			}
		}

		self
	}
}

pub fn setup(mut menu: ResMut<Menu>, mut commands: Commands, mut asset_server: Res<AssetServer>) {
	let style = font_style(&mut asset_server);

	let id = commands
		.spawn(
			NodeBundle {
				style: Style {
					display: Display::Flex,
					flex_direction: FlexDirection::Column,
					width: Val::Percent(100.0),
					height: Val::Percent(100.0),
					padding: UiRect::all(Val::Px(10.0)),

					..Default::default()
				},

				background_color: BackgroundColor(
					Color::rgba(
						0.0, 0.0, 0.0, 0.45,
					),
				),

				..Default::default()
			},
		)
		.id();

	let history = commands
		.spawn(
			NodeBundle {
				style: Style {
					display: Display::Flex,
					flex_direction: FlexDirection::Column,
					flex_grow: 10.0,
					width: Val::Percent(100.0),
					padding: UiRect::all(Val::Px(10.0)),
					overflow: Overflow::clip(),

					..Default::default()
				},

				background_color: BackgroundColor(
					Color::rgba(
						0.0, 0.0, 0.0, 0.55,
					),
				),

				..Default::default()
			},
		)
		.id();

	let input = commands
		.spawn(
			NodeBundle {
				style: Style {
					display: Display::Flex,
					align_items: AlignItems::Center,
					width: Val::Percent(100.0),
					height: Val::Px(14.0),
					padding: UiRect::all(Val::Px(10.0)),
					margin: UiRect::top(Val::Px(10.0)),

					..Default::default()
				},

				background_color: BackgroundColor(
					Color::rgba(
						0.0, 0.0, 0.0, 0.55,
					),
				),

				..Default::default()
			},
		)
		.id();

	let history_indicator = commands
		.spawn(
			(
				NodeBundle {
					style: Style {
						display: Display::Flex,
						flex_direction: FlexDirection::Column,
						width: Val::Percent(100.0),
						min_height: Val::Percent(100.0),

						..Default::default()
					},

					..Default::default()
				},
				AccessibilityNode(NodeBuilder::new(Role::List)),
			),
		)
		.id();

	let input_indicator = commands
		.spawn(
			TextBundle::from_section(
				"",
				style.clone(),
			),
		)
		.id();

	commands
		.entity(id)
		.push_children(&[history, input]);

	commands
		.entity(history)
		.add_child(history_indicator);

	commands
		.entity(input)
		.add_child(input_indicator);

	menu.save(
		id, input_indicator, history_indicator,
	);

	menu.history.restore(
		&style, &mut commands,
	);
}

pub fn cleanup(mut menu: ResMut<Menu>, mut commands: Commands) {
	if let Some(id) = menu.id {
		commands
			.entity(id)
			.despawn_recursive();

		menu.empty();
	}
}

pub fn listen_open(
	mut input_key: ResMut<Input<KeyCode>>,
	mut console_next_state: ResMut<NextState<ConsoleState>>,
) {
	if input_key.just_pressed(KeyCode::Grave) {
		console_next_state.set(ConsoleState::Open);

		input_key.clear();
	}
}

pub fn listen_close(
	menu: Res<Menu>,
	mut input_key: ResMut<Input<KeyCode>>,
	mut console_next_state: ResMut<NextState<ConsoleState>>,
) {
	if menu.input.is_empty() && input_key.any_just_pressed([KeyCode::Grave, KeyCode::Escape]) {
		console_next_state.set(ConsoleState::Close);

		input_key.clear();
	}
}

pub fn listen_text_input(
	mut menu: ResMut<Menu>,
	mut text_input_events: EventReader<TextInputEvent>,
	query: Query<&mut Text>,
) {
	for v in text_input_events.read() {
		menu.input
			.set(&v.message);
	}

	if menu.input.value.eq("`") {
		menu.input.clear();

		return;
	}

	menu.input.show(query);
}

pub fn listen_text_input_done(
	mut menu: ResMut<Menu>,
	mut commands: Commands,
	mut asset_server: Res<AssetServer>,
	mut text_input_done_events: EventReader<TextInputDoneEvent>,
	mut exit: EventWriter<AppExit>,
) {
	let style = font_style(&mut asset_server);

	for _v in text_input_done_events.read() {
		let message = menu.input.to_string();

		if message.eq("exit") {
			exit.send(AppExit);

			return;
		}

		if message.eq("clear") {
			menu.history
				.clear(&mut commands);

			return;
		}

		menu.input.clear();

		menu.history.pshow(
			&message, &style, &mut commands,
		);
	}
}

pub fn listen_text_input_cancel(
	mut menu: ResMut<Menu>,
	mut text_input_cancel_events: EventReader<TextInputCancelEvent>,
	mut console_next_state: ResMut<NextState<ConsoleState>>,
) {
	for _v in text_input_cancel_events.read() {
		if menu.input.is_empty() {
			console_next_state.set(ConsoleState::Close);

			return;
		}

		menu.input.clear();

		console_next_state.set(ConsoleState::Open);
	}
}

pub fn font_style(asset_server: &mut Res<AssetServer>) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/FiraMono-Medium.ttf"),
		font_size: 14.0,

		..Default::default()
	}
}
