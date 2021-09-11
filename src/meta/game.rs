use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// The metadata for the game as a whole.
#[derive(Serialize, Deserialize)]
pub struct Game {
	/// The human-friendly name for the game.
	pub name: String,
	/// A list of weeks, generated at runtime.
	#[serde(skip)]
	pub weeks: HashMap<String, super::Week>,
}
