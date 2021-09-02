use bevy::prelude::*;

use crate::{cleanup::*, level_select::*, menu::*, settings::*, GameState};

pub struct LevelSelect;

impl Plugin for LevelSelect {
	fn build(&self, app: &mut AppBuilder) {
		app.add_system_set(
			SystemSet::on_enter(GameState::LevelSelect)
				.with_system(cleanup.system())
				.with_system(setup.system()),
		)
		.add_system_set(
			SystemSet::on_update(GameState::LevelSelect)
				.with_system(menu_entry_choose_position.system())
				.with_system(menu_entry_set_position.system())
				.with_system(menu_entry_scale.system())
				.with_system(crate::menu::back_entry_open.system()),
		);
	}
}

fn setup(mut c: Commands, asset_server: Res<AssetServer>, mut s: ResMut<MenuSelected>) {
	s.0 = 0;
	mk_text_entry(&mut c, 0, &asset_server, "Level Select".to_string());
	mk_text_entry(&mut c, 1, &asset_server, "- Empty -".to_string());
	mk_back_entry(&mut c, 2, &asset_server);
	c.spawn_bundle(OrthographicCameraBundle::new_2d());
}
