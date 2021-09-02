// This module defines reusable menu components and systems.

use std::{thread, time::Duration};

use bevy::prelude::*;

pub struct MenuEntry(pub usize);

pub struct MenuSelected(pub usize);

pub fn menu_entry_set_position(
	mut e: Query<(&MenuEntry, &mut Transform)>,
	s: Res<MenuSelected>,
	ti: Res<Time>,
) {
	for (MenuEntry(n), mut t) in e.iter_mut() {
		let y_goal = (*n as isize - s.0 as isize) * -200;
		let goal =
			Transform::from_translation(Vec3::new(t.translation.x, y_goal as f32, t.translation.z));
		let distance = goal.translation - t.translation;
		t.translation += distance * ti.delta_seconds() * 10.0;
	}
}

pub fn menu_entry_scale(mut e: Query<(&MenuEntry, &mut Text)>, s: Res<MenuSelected>) {
	for (MenuEntry(n), mut text) in e.iter_mut() {
		if s.0 == *n {
			for text in &mut text.sections {
				text.style.font_size = 100.0;
			}
		} else {
			for text in &mut text.sections {
				text.style.font_size = 75.0;
			}
		}
	}
}

pub struct MenuChoose;

pub fn menu_entry_choose_position(
	mut e: EventWriter<MenuChoose>,
	mut s: ResMut<MenuSelected>,
	keys: Res<Input<KeyCode>>,
	entries: Query<&MenuEntry>,
) {
	if keys.just_pressed(KeyCode::Up) {
		s.0 = (s.0 as isize - 1).max(0) as usize;
	}
	if keys.just_pressed(KeyCode::Down) {
		s.0 += 1;
		let limit = entries.iter().map(|v| v.0).max().unwrap_or(0);
		if s.0 > limit {
			s.0 = limit
		}
	}
	if keys.just_released(KeyCode::Return) {
		println!("Choose {}", s.0);
		e.send(MenuChoose);
	}
}

pub fn mk_text_entry(
	c: &mut Commands,
	index: usize,
	asset_server: &Res<AssetServer>,
	text: String,
) {
	c.spawn_bundle(Text2dBundle {
		transform: Transform::from_translation(Vec3::ZERO),
		text: Text {
			sections: vec![TextSection {
				value: text,
				style: TextStyle {
					color: Color::WHITE,
					font_size: 75.0,
					font: asset_server.load("fonts/vcr.ttf"),
				},
			}],
			alignment: TextAlignment {
				vertical: VerticalAlign::Center,
				horizontal: HorizontalAlign::Center,
			},
		},
		..Default::default()
	})
	.insert(MenuEntry(index));
}

pub struct GrayOut;

pub fn gray_out(mut c: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
	c.spawn_bundle(SpriteBundle {
		sprite: Sprite::new(Vec2::new(f32::MAX, f32::MAX)),
		material: materials.add(ColorMaterial::color(Color::rgba_u8(0, 0, 0, 196))),
		..Default::default()
	})
	.insert(GrayOut);
}

pub fn un_gray_out(mut c: Commands, q: Query<Entity, With<GrayOut>>) {
	for e in q.iter() {
		c.entity(e).despawn();
	}
}

pub struct BackEntry;

pub fn mk_back_entry(c: &mut Commands, index: usize, asset_server: &Res<AssetServer>) {
	c.spawn_bundle(Text2dBundle {
		transform: Transform::from_translation(Vec3::ZERO),
		text: Text {
			sections: vec![TextSection {
				value: "Back".to_string(),
				style: TextStyle {
					color: Color::WHITE,
					font_size: 75.0,
					font: asset_server.load("fonts/vcr.ttf"),
				},
			}],
			alignment: TextAlignment {
				vertical: VerticalAlign::Center,
				horizontal: HorizontalAlign::Center,
			},
		},
		..Default::default()
	})
	.insert(BackEntry)
	.insert(MenuEntry(index));
}

pub fn back_entry_open(
	s: Res<MenuSelected>,
	mut e: EventReader<MenuChoose>,
	mut state: ResMut<State<crate::GameState>>,
	entry: Query<(&MenuEntry, &BackEntry)>,
) {
	for _ in e.iter() {
		for (MenuEntry(n), _) in entry.iter() {
			if s.0 == *n {
				println!("Select back");
				state.pop().unwrap();
			}
		}
	}
}
