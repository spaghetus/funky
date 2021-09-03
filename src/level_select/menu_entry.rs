use std::{thread, time::Duration};

use bevy::{prelude::*, text::Text2dSize};

use crate::{
	menu::{MenuChoose, MenuEntry, MenuSelected},
	GameState,
};

pub struct LSMenuEntry;

pub fn ls_menu_open(
	s: Res<MenuSelected>,
	mut e: EventReader<MenuChoose>,
	mut state: ResMut<State<GameState>>,
	entry: Query<&MenuEntry, With<LSMenuEntry>>,
) {
	for _ in e.iter() {
		for MenuEntry(n) in entry.iter() {
			if s.0 == *n {
				println!("Select level select");
				state.push(GameState::LevelSelect).unwrap();
				return;
			}
		}
	}
}

pub fn mk_ls_menu_entry(
	c: &mut Commands,
	index: usize,
	asset_server: &Res<AssetServer>,
	windows: &Res<Windows>,
	state: &Res<State<GameState>>,
) {
	c.spawn_bundle(Text2dBundle {
		transform: Transform::from_translation(Vec3::ZERO),
		text: Text {
			sections: vec![TextSection {
				value: "Play".to_string(),
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
		text_2d_size: Text2dSize {
			size: Size {
				width: windows.get_primary().unwrap().width() / 2.0,
				height: f32::MAX,
			},
		},
		..Default::default()
	})
	.insert(LSMenuEntry)
	.insert(state.current().clone())
	.insert(MenuEntry(index));
}
