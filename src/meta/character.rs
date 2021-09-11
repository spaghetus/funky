/// The metadata for a character.
pub struct Character {
	/// The human-friendly name for a character.
	pub name: String,
	/// The unlocks required for this character to be used in a possible future "free play" mode.
	pub unlock_groups: Option<Vec<String>>,
}
