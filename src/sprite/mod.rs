//! This file contains sprite animation and management.
#![allow(dead_code)]

use bevy::{ecs::system::EntityCommands, prelude::*};
use std::collections::BTreeMap;

pub struct Animated;

pub struct AnimationTimer(pub usize);

pub struct CurrentAnimation(pub String);

pub struct AnimationSet {
	pub animations: BTreeMap<String, Animation>,
}

pub struct AnimatedSpriteTransform(pub Transform);

pub type Animation = BTreeMap<usize, Frame>;

pub struct Frame {
	pub atlas_index: u32,
	pub position: Vec2,
}

impl<'a> Animated {
	pub fn mk_animated_sprite(
		c: &mut EntityCommands,
		animation_set: AnimationSet,
		atlas: Handle<TextureAtlas>,
		starting_animation: String,
	) {
		c.insert_bundle(SpriteSheetBundle {
			texture_atlas: atlas,
			..Default::default()
		})
		.insert(Animated)
		.insert(animation_set)
		.insert(CurrentAnimation(starting_animation));
	}

	/// This system sets a sprite's AnimatedSpriteTransform on its first frame of existence.
	pub fn set_special_transform(
		mut c: Commands,
		q: Query<(Entity, &Transform), (With<Animated>, Without<AnimatedSpriteTransform>)>,
	) {
		for (e, t) in q.iter() {
			c.entity(e).insert(AnimatedSpriteTransform(*t));
		}
	}

	/// This system increments a sprite's timer every frame.
	pub fn sprite_timer(
		mut q: Query<&mut AnimationTimer, (With<Animated>, With<AnimatedSpriteTransform>)>,
	) {
		for mut t in q.iter_mut() {
			t.0 += 1;
		}
	}

	/// This system sets a sprite's image to the correct image for its frame.
	pub fn animate_sprites(
		mut q: Query<
			(
				&mut TextureAtlasSprite,
				&TextureAtlas,
				&mut Transform,
				&AnimationTimer,
				&CurrentAnimation,
				&AnimationSet,
				&AnimatedSpriteTransform,
			),
			With<Animated>,
		>,
	) {
		for (mut s, te, mut tr, ti, c, a, atr) in q.iter_mut() {
			let animation = match a.animations.get(&c.0) {
				Some(v) => v,
				None => {
					eprintln!(
						"Attempted to play an animation ({}) that doesn't exist.",
						c.0
					);
					continue;
				}
			};
			let (_, frame) = animation
				.iter()
				.filter(|(n, _)| n < &&ti.0)
				.last()
				.unwrap_or_else(|| animation.iter().next().unwrap());
			let top_left = te.textures[frame.atlas_index as usize].min;
			let top_right = te.textures[frame.atlas_index as usize].max;
			let true_center = frame.position;
			let sprite_center = (top_left + top_right) / 2.0;
			let offset = -(true_center - sprite_center) * Vec2::new(tr.scale.x, tr.scale.y);
			s.index = frame.atlas_index;
			tr.translation = atr.0.translation + Vec3::new(offset.x, offset.y, 0.0);
		}
	}
}
