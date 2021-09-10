#![allow(incomplete_features)]
#![feature(const_generics)]

use bevy::prelude::*;
mod cleanup;
mod level_select;
mod menu;
mod settings;
mod title;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
	Title,
	Settings,
	LevelSelect,
	Loading,
	Game,
	Paused,
}

fn main() {
	App::build()
		.add_plugins(DefaultPlugins)
		.add_plugin(title::Title)
		.add_plugin(level_select::LevelSelect)
		.add_plugin(settings::Settings)
		.add_state(GameState::Title)
		.add_event::<crate::menu::MenuChoose>()
		.insert_resource(crate::menu::MenuSelected(1))
		.add_startup_system(setup.system())
		.run();
}
