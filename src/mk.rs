#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![forbid(missing_docs)]

//! This file is the entrypoint for make-funky.

pub use midi_to_hell::convert;
pub use rayon::prelude::*;
pub use std::{fs::read_to_string, path::PathBuf};
use std::{fs::File, io::BufReader};
use xml_dom::level2::{Element, Node, RefNode};
pub mod meta;
pub use meta::*;
pub use walkdir::WalkDir;

fn main() {
	WalkDir::new("game")
		.into_iter()
		// .par_bridge()
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
					Some("char") => match ron::from_str::<Character>(
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
					Some("svg") => {
						// Read svg into memory
						let tree = std::fs::read_to_string(src_path).unwrap();
						// Build DOM tree
						let tree = xml_dom::parser::read_xml(&tree).unwrap();
						let animations = dom_search(tree, &|node| {
							node.node_name().to_string() == "g"
								&& node
									.get_attribute("id")
									.unwrap_or("".to_string())
									.starts_with("ANIM ")
						});
						// (Animation root, [frame root])
						let animations: Vec<(RefNode, Vec<RefNode>)> = animations
							.iter()
							.map(|animation| {
								(
									animation.clone(),
									dom_search(animation.clone(), &|node| {
										node.node_name().to_string() == "g"
											&& node
												.get_attribute("id")
												.unwrap_or("".to_string())
												.split(" ")
												.filter(|v| v.parse::<usize>().is_err())
												.count() == 0
									}),
								)
							})
							.collect();
						println!(
							"{} animation(s) found, with {} frames total.",
							animations.len(),
							animations.iter().flat_map(|(_, f)| f).count()
						);
						// (Animation root, [(Frame root, rect, origin)])
						let animations: Vec<(RefNode, Vec<(RefNode, RefNode, RefNode)>)> =
							animations
								.iter()
								.map(|(animation, frames)| {
									(
										animation.clone(),
										frames
											.iter()
											.map(|frame| {
												(
													frame.clone(),
													dom_search(frame.clone(), &|node| {
														node.node_name().to_string() == "rect"
													})
													.iter()
													.next()
													.expect("Frame missing atlas rect")
													.clone(),
													dom_search(frame.clone(), &|node| {
														node.node_name().to_string() == "path"
													})
													.iter()
													.next()
													.expect("Frame missing atlas center")
													.clone(),
												)
											})
											.collect(),
									)
								})
								.collect();
						// (Animation root, [(Frame root, AABB, coordinates)])
						let animations: Vec<(RefNode, Vec<(RefNode, [usize; 4], [usize; 2])>)> =
							animations
								.iter()
								.map(|(animation, frames)| {
									(
										animation.clone(),
										frames
											.iter()
											.map(|(root, rect, origin)| {
												(
													root.clone(),
													[
														rect.get_attribute("x")
															.unwrap()
															.parse::<f64>()
															.unwrap()
															.floor() as usize,
														rect.get_attribute("y")
															.unwrap()
															.parse::<f64>()
															.unwrap()
															.floor() as usize,
														rect.get_attribute("width")
															.unwrap()
															.parse::<f64>()
															.unwrap()
															.ceil() as usize,
														rect.get_attribute("height")
															.unwrap()
															.parse::<f64>()
															.unwrap()
															.ceil() as usize,
													],
													[
														origin
															.get_attribute("sodipodi:cx")
															.unwrap()
															.parse::<f64>()
															.unwrap()
															.round() as usize,
														origin
															.get_attribute("sodipodi:cy")
															.unwrap()
															.parse::<f64>()
															.unwrap()
															.round() as usize,
													],
												)
											})
											.collect(),
									)
								})
								.collect();
						let animations = GeneratedAtlas {
							animations: animations
								.iter()
								.map(|(animation, frames)| {
									(
										animation.get_attribute("id").unwrap().to_string()[5..]
											.to_string(),
										frames
											.iter()
											.flat_map(|(node, aabb, origin)| {
												node.get_attribute("id")
													.unwrap_or("".to_string())
													.split(" ")
													.map(|v| v.parse::<usize>().unwrap())
													.map(move |n| (n, (*aabb, *origin)))
													.collect::<Vec<(usize, ([usize; 4], [usize; 2]))>>(
													)
											})
											.collect(),
									)
								})
								.collect(),
						};
						let atlas_path = PathBuf::from(dest_path.clone())
							.parent()
							.unwrap()
							.join("meta.atlas");
						std::fs::write(atlas_path, ron::to_string(&animations).unwrap()).unwrap();
						false
					}
					_ => false,
				};
				std::fs::create_dir_all(PathBuf::from(dest_path.clone()).parent().unwrap())
					.unwrap();
				if should_copy {
					println!("{} -> {}", src_path, dest_path);
					std::fs::copy(src_path, &dest_path).unwrap();
				}
			}
			_ => {}
		});
}

fn dom_search(from: RefNode, check: &dyn Fn(RefNode) -> bool) -> Vec<RefNode> {
	if check(from.clone()) {
		vec![from]
	} else {
		from.child_nodes()
			.iter()
			.flat_map(|v| dom_search(v.clone(), check))
			.collect()
	}
}
