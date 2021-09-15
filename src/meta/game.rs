use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

/// The metadata for the game as a whole.
#[derive(Serialize, Deserialize)]
pub struct Game {
	/// The human-friendly name for the game.
	pub name: String,
	/// An ordered list of weeks, generated at runtime.
	#[serde(skip)]
	pub weeks: Vec<super::Week>,
	/// A list of characters, generated at runtime.
	#[serde(skip)]
	pub characters: HashMap<String, super::Character>,
	/// The path of the game folder.
	#[serde(skip)]
	pub path: PathBuf,
}

use anyhow::Result;
use std::fs;

use crate::meta::Week;

impl Game {
	pub fn load(path: &str) -> Result<Game> {
		let game = fs::read_to_string(PathBuf::from(path).join("meta.game"))?;
		let mut game: Game = ron::from_str(&game)?;
		let mut weeks: Vec<_> = fs::read_dir(PathBuf::from(path).join("weeks"))?
			.into_iter()
			.map(|v| {
				let v = v.unwrap();
				let index: usize = v.file_name().to_string_lossy().parse().unwrap();
				let week = Week::load(v.path().to_str().unwrap()).unwrap();
				(index, week)
			})
			.collect();
		weeks.sort_by(|(a, _), (b, _)| a.cmp(b));
		game.weeks = weeks.iter().map(|(_, v)| v.clone()).collect();
		game.path = PathBuf::from(path);
		Ok(game)
	}
}
