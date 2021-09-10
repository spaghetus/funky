use bevy::prelude::*;

use crate::GameState;

pub struct Game;

impl Plugin for Game {
	fn build(&self, app: &mut AppBuilder) {
		app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup.system()));
	}
}

fn setup() {}
