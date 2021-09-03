use bevy::prelude::*;

use crate::{menu::*, GameState};

pub struct LevelSelect;

impl Plugin for LevelSelect {
	fn build(&self, app: &mut AppBuilder) {
		app.add_system_set(SystemSet::on_enter(GameState::LevelSelect).with_system(setup.system()))
			.add_system_set(
				SystemSet::on_update(GameState::LevelSelect)
					.with_system(menu_entry_choose_position.system())
					.with_system(menu_entry_set_position.system())
					.with_system(menu_entry_scale.system())
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
	mk_text_entry(
		&mut c,
		1,
		&asset_server,
		"- Empty -".to_string(),
		&windows,
		&state,
	);
	mk_back_entry(&mut c, 2, &asset_server, &state);
	c.spawn_bundle(OrthographicCameraBundle::new_2d());
}
