#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![forbid(missing_docs)]
#![allow(clippy::type_complexity)]

//! This file is the entrypoint for make-funky.

pub use midi_to_hell::convert;
pub use rayon::prelude::*;
pub use std::{fs::read_to_string, path::PathBuf};
use xml_dom::level2::{Element, Node, RefNode};
pub mod meta;
use meta::*;
pub use walkdir::WalkDir;

const SCALE: usize = 2;

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
				let should_copy = match v.file_name().to_string_lossy().split('.').last() {
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
						// Get the list of animations and their frames
						let animations: Vec<(RefNode, Vec<(RefNode, [usize; 4], [usize; 2])>)> =
							// Get every animation
							dom_search(tree.clone(), &|node| {
								node.node_name().to_string() == "g"
									&& node
										.get_attribute("id")
										.unwrap_or_else(||"".to_string())
										.starts_with("ANIM ")
							})
							.iter()
							// Vec<RefNode>
							// Get every frame of every animation
							.map(|animation| {
								(
									animation.clone(),
									dom_search(animation.clone(), &|node| {
										node.node_name().to_string() == "g"
											&& node
											.get_attribute("id")
												.unwrap_or_else(||"".to_string())
												.split(' ')
												.filter(|v| v.parse::<usize>().is_err())
												.count() == 0
											}),
										)
									})
							// Vec<(RefNode, Vec<RefNode>)>
							// Get every frame of every animation
							.map(|(animation, frames)| {
								(
									animation,
									frames
										.iter()
										.map(|frame| {
											(
												frame.clone(),
												dom_search(frame.clone(), &|node| {
													node.node_name().to_string() == "rect"
												}).get(0)
												.expect("Frame missing atlas rect")
												.clone(),
												dom_search(frame.clone(), &|node| {
													node.node_name().to_string() == "path"
												}).get(0)
												.expect("Frame missing atlas center")
												.clone(),
											)
										})
										.collect(),
								)
							})
							// Vec<(RefNode, Vec<(RefNode, RefNode, RefNode)>)>
							// Get the AABB and origin of every frame
							.map(
								|(animation, frames): (
									RefNode,
									Vec<(RefNode, RefNode, RefNode)>,
								)| {
									(
										animation,
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
															.floor() as usize * SCALE,
														rect.get_attribute("y")
															.unwrap()
															.parse::<f64>()
															.unwrap()
															.floor() as usize * SCALE,
														rect.get_attribute("width")
															.unwrap()
															.parse::<f64>()
															.unwrap()
															.ceil() as usize * SCALE,
														rect.get_attribute("height")
															.unwrap()
															.parse::<f64>()
															.unwrap()
															.ceil() as usize * SCALE,
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
								},
							)
							.collect();
						// Vec<(RefNode, Vec<(RefNode, [usize; 4], [usize; 2])>)>
						// Build the GeneratedAtlas of every animation
						let animations = GeneratedAtlas {
							animations: animations
								.iter()
								.map(|(animation, frames)| {
									(
										animation.get_attribute("id").unwrap()[5..].to_string(),
										frames
											.iter()
											.flat_map(|(node, aabb, origin)| {
												node.get_attribute("id")
													.unwrap_or_else(|| "".to_string())
													.split(' ')
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
						// Build the target path for the GeneratedAtlas
						let atlas_path = PathBuf::from(dest_path.clone())
							.parent()
							.unwrap()
							.join("meta.atlas");
						// Write out the GeneratedAtlas
						std::fs::write(atlas_path, ron::to_string(&animations).unwrap()).unwrap();
						// Build the target path of the PNG
						let atlas_path = PathBuf::from(dest_path.clone())
							.parent()
							.unwrap()
							.join("atlas.png");
						dom_search(tree.clone(), &|node| {
							if node.node_name().to_string() == "g" {
								node.get_attribute("id")
									.unwrap_or_else(|| "".to_string())
									.starts_with("ANIM ")
							} else {
								false
							}
						})
						.iter_mut()
						.for_each(|node| node.set_attribute("visibility", "hidden").unwrap());
						let tree = usvg::Tree::from_str(
							&tree.to_string(),
							&usvg::Options::default().to_ref(),
						)
						.unwrap();
						let pixmap_size = tree.svg_node().size.to_screen_size();
						let mut pixmap = tiny_skia::Pixmap::new(
							pixmap_size.width() * SCALE as u32,
							pixmap_size.height() * SCALE as u32,
						)
						.unwrap();
						resvg::render(
							&tree,
							usvg::FitTo::Size(pixmap.width(), pixmap.height()),
							pixmap.as_mut(),
						);
						pixmap.save_png(atlas_path).unwrap();
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
