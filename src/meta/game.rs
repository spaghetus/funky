use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Game {
	pub name: String,
	#[serde(skip)]
	pub weeks: HashMap<String, super::Week>,
}
