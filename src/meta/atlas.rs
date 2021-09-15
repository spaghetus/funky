use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// The metadata for a sprite atlas
#[derive(Serialize, Deserialize)]
pub struct GeneratedAtlas {
	/// The rects for the atlas's various animations.
	pub animations: BTreeMap<String, HashMap<usize, ([usize; 4], [usize; 2])>>,
}
