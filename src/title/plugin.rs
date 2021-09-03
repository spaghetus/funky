use bevy::prelude::*;

use crate::{cleanup::*, level_select::*, menu::*, settings::*, GameState};

pub struct Title;

impl Plugin for Title {
	fn build(&self, app: &mut AppBuilder) {
		app.add_system_set(
			SystemSet::on_enter(GameState::Title)
				.with_system(cleanup.system())
				.with_system(setup.system()),
		)
		.add_system_set(
			SystemSet::on_update(GameState::Title)
				.with_system(menu_entry_choose_position.system())
				.with_system(menu_entry_set_position.system())
				.with_system(menu_entry_scale.system())
				.with_system(ls_menu_open.system())
				.with_system(settings_menu_open.system()),
		)
		.add_system_set(SystemSet::on_pause(GameState::Title).with_system(gray_out.system()))
		.add_system_set(SystemSet::on_resume(GameState::Title).with_system(un_gray_out.system()));
	}
}

fn setup(
	mut c: Commands,
	asset_server: Res<AssetServer>,
	windows: Res<Windows>,
	state: Res<State<GameState>>,
) {
	mk_text_entry(
		&mut c,
		0,
		&asset_server,
		"Funky Engine".to_string(),
		&windows,
		&state,
	);
	mk_ls_menu_entry(&mut c, 1, &asset_server, &windows, &state);
	mk_settings_menu_entry(&mut c, 2, &asset_server, &state);
	c.spawn_bundle(OrthographicCameraBundle::new_2d());
}
