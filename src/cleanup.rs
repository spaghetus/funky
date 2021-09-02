use bevy::prelude::{Commands, Entity, Query};

pub fn cleanup(mut c: Commands, e: Query<Entity>) {
	for e in e.iter() {
		c.entity(e).despawn();
	}
}
