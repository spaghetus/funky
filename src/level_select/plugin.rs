use bevy::prelude::*;

use crate::{
	game::{game_entry_open, mk_game_menu_entry},
	menu::*,
	meta::Game,
	GameState,
};

pub struct LevelSelect;

impl Plugin for LevelSelect {
	fn build(&self, app: &mut AppBuilder) {
		app.add_system_set(SystemSet::on_enter(GameState::LevelSelect).with_system(setup.system()))
			.add_system_set(
				SystemSet::on_update(GameState::LevelSelect)
					.with_system(menu_entry_choose_position.system())
					.with_system(menu_entry_set_position.system())
					.with_system(menu_entry_scale.system())
					.with_system(game_entry_open.system())
					.with_system(back_entry_open.system()),
			)
			.add_system_set(
				SystemSet::on_exit(GameState::LevelSelect)
					.with_system(cleanup_entries::<{ GameState::LevelSelect }>.system()),
			);
	}
}

fn setup(
	mut c: Commands,
	asset_server: Res<AssetServer>,
	mut s: ResMut<MenuSelected>,
	windows: Res<Windows>,
	state: Res<State<GameState>>,
	game: Res<Game>,
) {
	s.0 = 0;
	mk_text_entry(
		&mut c,
		0,
		&asset_server,
		"Level Select".to_string(),
		&windows,
		&state,
	);
	let mut index = 1;
	for week in &game.weeks {
		mk_text_entry(
			&mut c,
			index,
			&asset_server,
			format!("- {} -", week.name.clone()),
			&windows,
			&state,
		);
		index += 1;
		for song in &week.songs {
			mk_game_menu_entry(&mut c, index, &asset_server, &windows, &state, &song);
			index += 1
		}
	}
	mk_back_entry(&mut c, index, &asset_server, &state);
}
