//! This module defines the structure of the metadata files stored in /game.

use std::path::PathBuf;

mod game;
pub use game::Game;
mod song;
pub use song::Song;
mod week;
pub use week::Week;
mod character;
pub use character::Character;
mod atlas;
pub use atlas::GeneratedAtlas;

pub fn path_under_assets(path: &PathBuf) -> PathBuf {
	path.clone()
		.into_iter()
		.skip_while(|v| v.to_str() != Some("assets"))
		.skip(1)
		.collect()
}

macro_rules! play_looped {
	($audio:ident, $asset_server:ident, $path:expr) => {
		$audio.stop();
		$audio.play_looped(
			$asset_server.load(crate::meta::path_under_assets(&$path).to_str().unwrap()),
		)
	};
}
