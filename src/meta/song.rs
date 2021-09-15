use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// The metadata for an individual song.
#[derive(Serialize, Deserialize, Clone)]
pub struct Song {
	/// The human-friendly name of a song.
	pub name: String,
	/// The list of player characters in a song.
	/// Players beyond this list will be randomly chosen.
	pub players: Vec<String>,
	/// The list of enemies in a song.
	pub enemies: Vec<String>,
	/// The name of the singleplayer cue track.
	pub singleplayer_cue_name: String,
	/// The names of the multiplayer cue tracks.
	/// Players beyond this list will wrap.
	pub cue_names: Vec<String>,
	/// The names of the enemies' cue tracks.
	pub enemy_cue_names: Vec<String>,
	/// Unused right now.
	/// Will allow specifying extra Systems to load.
	pub extra_plugins: Vec<String>,
	/// Suffixes for the different difficulty levels.
	pub difficulty_suffixes: HashMap<String, String>,
	/// The groups this song requires.
	pub requires: Vec<String>,
	/// The groups this song unlocks upon completion.
	pub unlocks: Vec<String>,
	/// The extra groups this song unlocks upon *perfect* completion.
	pub perfect_unlocks: Vec<String>,
	/// The path of the song folder.
	#[serde(skip)]
	pub path: PathBuf,
}

use anyhow::Result;
use std::fs;
use std::path::PathBuf;

impl Song {
	pub fn load(path: &str) -> Result<Song> {
		let song = fs::read_to_string(PathBuf::from(path).join("meta.song"))?.to_string();
		let mut song: Song = ron::from_str(&song)?;
		song.path = PathBuf::from(path);
		Ok(song)
	}
}
