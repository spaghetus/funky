use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Week {
	pub name: String,
	pub left_title_characters: Vec<String>,
	pub right_title_characters: Vec<String>,
	pub title_color: [u8; 3],
	#[serde(skip)]
	pub songs: Vec<super::Song>,
}
