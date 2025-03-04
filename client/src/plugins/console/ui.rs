use bevy::{
	asset::AssetServer,
	ecs::{
		entity::Entity,
		event::EventWriter,
		query::With,
		system::{Commands, In, Query, Res, ResMut},
	},
	hierarchy::{BuildChildren as _, DespawnRecursiveExt as _},
	input::{keyboard::KeyCode, ButtonInput},
	state::state::NextState,
	text::TextFont,
	ui::{AlignItems, Display, FlexDirection, JustifyContent, Node, Overflow, UiRect, Val},
	window::Window,
};

use super::{
	event::{PanelRefresh, PromptRefresh},
	panel::{Label, Panel},
	prompt::Prompt,
	state::Console,
};
use crate::core::event::Execute;

pub fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut window: Query<&mut Window>,
	mut panel_refresh: EventWriter<PanelRefresh>,
) {
	let window = window.single_mut();


	let panel = Panel::new(
		window.height(),
		TextFont {
			font: asset_server.load("fonts/SourceHanSansCN-Regular.otf"),
			font_size: 16.0,

			..Default::default()
		},
	);

	let input = panel.from_section("$ _");


	let mut label: Vec<Entity> = Vec::new();

	for _ in 0..panel.max_display {
		let mut label_text = panel.from_section("");

		label_text.1.font_size = 10.0;
		label_text.3.margin = UiRect::left(Val::Px(10.0));

		let id = commands
			.spawn(
				(
					Label, label_text,
				),
			)
			.id();

		label.push(id);
	}

	let root = commands
		.spawn(
			(
				panel,
				Node {
					display: Display::Flex,
					flex_direction: FlexDirection::Column,
					justify_content: JustifyContent::FlexEnd,
					width: Val::Vw(100.0),
					height: Val::Vh(100.0),
					padding: UiRect::all(Val::Px(10.0)),
					overflow: Overflow::clip(),

					..Default::default() // background_color: BackgroundColor(
					                     // 	Color::srgba(
					                     // 		0.0, 0.0, 0.0, 0.75,
					                     // 	),
					                     // ),
				},
			),
		)
		.id();

	let prompt = commands
		.spawn(
			(
				Prompt,
				input,
				Node {
					display: Display::Flex,
					align_items: AlignItems::Center,
					width: Val::Percent(100.0),
					margin: UiRect::top(Val::Px(8.0)),

					..Default::default()
				},
			),
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

pub fn open(mut key_code: ResMut<ButtonInput<KeyCode>>, mut console: ResMut<NextState<Console>>) {
	if key_code.just_pressed(KeyCode::Backquote) {
		console.set(Console::Open);

		key_code.clear();
	}
}

pub fn close(
	mut key_code: ResMut<ButtonInput<KeyCode>>,
	mut console: ResMut<NextState<Console>>,
	mut prompt_refresh: EventWriter<PromptRefresh>,
) {
	if key_code.any_just_pressed([KeyCode::Backquote, KeyCode::Escape]) {
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
