use bevy::{prelude::*, text::Text2dSize};

use crate::{
	menu::{MenuChoose, MenuEntry},
	meta::Song,
	GameState,
};

pub struct GameMenuEntry;

pub fn game_entry_open(
	mut e: EventReader<MenuChoose>,
	mut state: ResMut<State<GameState>>,
	entry: Query<(&MenuEntry, &GameState, &Song), With<GameMenuEntry>>,
) {
	for MenuChoose(item, ev_state) in e.iter() {
		for (s, en_state, song) in entry.iter() {
			if s.0 == *item && en_state == ev_state {
				// state.push(GameState::LevelSelect).unwrap();
				println!("{}", song.name);
				println!("doesn't exist yet lol")
			}
		}
	}
}

pub fn mk_game_menu_entry(
	c: &mut Commands,
	index: usize,
	asset_server: &Res<AssetServer>,
	windows: &Res<Windows>,
	state: &Res<State<GameState>>,
	song: &Song,
) {
	c.spawn_bundle(Text2dBundle {
		transform: Transform::from_translation(Vec3::ZERO),
		text: Text {
			sections: vec![TextSection {
				value: song.name.clone(),
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
	.insert(GameMenuEntry)
	.insert(state.current().clone())
	.insert(MenuEntry(index))
	.insert(song.clone());
}
