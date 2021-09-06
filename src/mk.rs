// This file is the entrypoint for make-funky.

use std::{
	fs::read_to_string,
	path::{Path, PathBuf},
};

use anyhow::Error;
use midi_to_hell::convert;
use rayon::prelude::*;

mod meta;
use meta::*;
use walkdir::WalkDir;

pub fn main() {
	WalkDir::new("game")
		.into_iter()
		.par_bridge()
		.filter_map(Result::ok)
		.for_each(|v| match v.file_type() {
			t if t.is_file() => {
				let src_path = &*v.path().to_string_lossy();
				// let src_path = format!("game/{}", src_path);
				let dest_path = format!("assets/{}", src_path);
				let should_copy = match v.file_name().to_string_lossy().split(".").last() {
					Some("game") => match ron::from_str::<Game>(
						&read_to_string(v.path().to_str().unwrap()).unwrap(),
					) {
						Ok(_) => true,
						Err(e) => {
							panic!(
								"Failed to serialize {} with {}",
								v.path().to_str().unwrap(),
								e
							)
						}
					},
					Some("week") => match ron::from_str::<Week>(
						&read_to_string(v.path().to_str().unwrap()).unwrap(),
					) {
						Ok(_) => true,
						Err(e) => {
							panic!(
								"Failed to serialize {} with {}",
								v.path().to_str().unwrap(),
								e
							)
						}
					},
					Some("song") => match ron::from_str::<Song>(
						&read_to_string(v.path().to_str().unwrap()).unwrap(),
					) {
						Ok(_) => true,
						Err(e) => {
							panic!(
								"Failed to serialize {} with {}",
								v.path().to_str().unwrap(),
								e
							)
						}
					},
					Some("wav" | "txt" | "md") => true,
					Some("mid" | "midi") => {
						// Wacky MIDI transformation happens here
						let dest_path =
							dest_path.replace(".midi", ".hell").replace(".mid", ".hell");
						println!("{} -> midi-to-hell -> {}", src_path, dest_path);
						let input = std::fs::read(src_path).unwrap();
						let output = ron::to_string(&convert("fnf", &input)).unwrap();
						std::fs::write(dest_path, output).unwrap();
						false
					}
					_ => false,
				};
				if should_copy {
					println!("{} -> {}", src_path, dest_path);
					std::fs::create_dir_all(PathBuf::from(dest_path.clone()).parent().unwrap())
						.unwrap();
					std::fs::copy(src_path, &dest_path).unwrap();
				}
			}
			_ => {}
		});
}
