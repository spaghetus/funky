use crate::meta::Song;
use serde::{Deserialize, Serialize};

/// The metadata for a week.
#[derive(Serialize, Deserialize, Clone)]
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
	pub songs: Vec<Song>,
	/// The path of the week folder.
	#[serde(skip)]
	pub path: PathBuf,
}

use anyhow::Result;
use std::fs;
use std::path::PathBuf;

impl Week {
	pub fn load(path: &str) -> Result<Week> {
		let week = fs::read_to_string(PathBuf::from(path).join("meta.week"))?.to_string();
		let mut week: Week = ron::from_str(&week)?;
		let mut songs: Vec<_> = fs::read_dir(PathBuf::from(path).join("songs"))?
			.into_iter()
			.map(|v| {
				let v = v.unwrap();
				let index: usize = v.file_name().to_string_lossy().parse().unwrap();
				let week = Song::load(v.path().to_str().unwrap()).unwrap();
				(index, week)
			})
			.collect();
		songs.sort_by(|(a, _), (b, _)| a.cmp(b));
		week.songs = songs.iter().map(|(_, v)| v.clone()).collect();
		week.path = PathBuf::from(path);
		Ok(week)
	}
}
