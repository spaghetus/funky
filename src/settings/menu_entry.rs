use crate::menu::{MenuChoose, MenuEntry};
use crate::GameState;
use bevy::prelude::*;

pub struct SettingsMenuEntry;

pub fn settings_menu_open(
	mut e: EventReader<MenuChoose>,
	mut state: ResMut<State<GameState>>,
	entry: Query<(&MenuEntry, &GameState), With<SettingsMenuEntry>>,
) {
	for MenuChoose(item, ev_state) in e.iter() {
		for (s, en_state) in entry.iter() {
			if s.0 == *item && en_state == ev_state {
				state.push(GameState::Settings).unwrap();
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
