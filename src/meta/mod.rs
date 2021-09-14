//! This module defines the structure of the metadata files stored in /game.

mod game;
pub use game::Game;
mod song;
pub use song::Song;
mod week;
pub use week::Week;
mod character;
pub use character::Character;
mod atlas;
pub use atlas::GeneratedAtlas;
