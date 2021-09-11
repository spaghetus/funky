use serde::{Deserialize, Serialize};

/// The metadata for a week.
#[derive(Serialize, Deserialize)]
pub struct Week {
	/// The human-friendly name for the week.
	pub name: String,
	/// The characters shown on the left of the screen in the level select.
	pub left_title_characters: Vec<String>,
	/// The characters shown on the right of the screen in the level select.
	pub right_title_characters: Vec<String>,
	/// The background color in the level select.
	pub title_color: [u8; 3],
	/// The list of songs in the week. Generated at runtime.
	#[serde(skip)]
	pub songs: Vec<super::Song>,
}
