use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Game {
	pub name: String,
	#[serde(skip)]
	pub weeks: Vec<super::Week>,
}
