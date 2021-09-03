use bevy::prelude::Commands;

// This module defines the settings menu.

use bevy::prelude::*;

use crate::menu::MenuSelected;
use crate::GameState;

pub struct SettingsMenuEntry;

pub fn settings_menu_open(
	mut c: Commands,
	s: Res<MenuSelected>,
	mut e: EventReader<crate::menu::MenuChoose>,
	mut state: ResMut<State<crate::GameState>>,
	entry: Query<&crate::menu::MenuEntry, With<SettingsMenuEntry>>,
) {
	for _ in e.iter() {
		for crate::menu::MenuEntry(n) in entry.iter() {
			if s.0 == *n {
				println!("Select settings");
				state.push(crate::GameState::Settings).unwrap();
			}
		}
	}
}

pub fn mk_settings_menu_entry(
	c: &mut Commands,
	index: usize,
	asset_server: &Res<AssetServer>,
	state: &Res<State<GameState>>,
) {
	c.spawn_bundle(Text2dBundle {
		transform: Transform::from_translation(Vec3::ZERO),
		text: Text {
			sections: vec![TextSection {
				value: "Settings".to_string(),
				style: TextStyle {
					color: Color::WHITE,
					font_size: 75.0,
					font: asset_server.load("fonts/vcr.ttf"),
				},
			}],
			alignment: TextAlignment {
				vertical: VerticalAlign::Center,
				horizontal: HorizontalAlign::Center,
			},
		},
		..Default::default()
	})
	.insert(SettingsMenuEntry)
	.insert(state.current().clone())
	.insert(crate::menu::MenuEntry(index));
}
