//! This file contains systems which clean the world.

use bevy::prelude::{Commands, Entity, Query};

pub fn cleanup(mut c: Commands, e: Query<Entity>) {
	for e in e.iter() {
		c.entity(e).despawn();
	}
}
