use bevy;
use bevy::asset::AssetServer;
use bevy::ecs::system::Commands;
use bevy::ecs::system::Res;
use bevy::hierarchy::BuildChildren;
use bevy::render::color::Color;
use bevy::text::TextStyle;

use bevy::ui::node_bundles::NodeBundle;
use bevy::ui::node_bundles::TextBundle;
use bevy::ui::widget::Label;
use bevy::ui::Style;
use bevy::ui::UiRect;
use bevy::ui::Val;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	// root node
	commands
		.spawn(
			NodeBundle {
				style: Style {
					width: Val::Percent(100.0),
					height: Val::Percent(72.0),
					..bevy::prelude::default()
				},
				background_color: Color::rgba(
					0.15, 0.15, 0.15, 0.45,
				)
				.into(),
				..bevy::prelude::default()
			},
		)
		.with_children(
			|parent| {
				// text
				parent.spawn(
					(
						TextBundle::from_section(
							"Text Example",
							TextStyle {
								font: asset_server.load("fonts/FiraSans-Bold.ttf"),
								font_size: 30.0,
								..bevy::prelude::default()
							},
						)
						.with_style(
							Style {
								margin: UiRect::all(Val::Px(5.)),
								..bevy::prelude::default()
							},
						),
						// Because this is a distinct label widget and
						// not button/list item text, this is necessary
						// for accessibility to treat the text accordingly.
						Label,
					),
				);
			},
		);
}
