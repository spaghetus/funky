use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Song {
	pub name: String,
	pub players: Vec<String>,
	pub enemies: Vec<String>,
	pub singleplayer_cue_name: String,
	pub cue_names: Vec<String>,
	pub enemy_cue_names: Vec<String>,
	pub extra_plugins: Vec<String>,
	pub difficulty_suffixes: HashMap<String, String>,
}
