//! This module defines reusable menu components and systems.

use bevy::{prelude::*, text::Text2dSize};

use crate::GameState;

pub struct MenuEntry(pub usize);

pub struct MenuSelected(pub usize);

pub fn menu_entry_set_position(
	mut e: Query<(&MenuEntry, &mut Transform, &GameState)>,
	state: Res<State<GameState>>,
	s: Res<MenuSelected>,
	ti: Res<Time>,
) {
	for (MenuEntry(n), mut t, e_state) in e.iter_mut() {
		let (distance, coefficient, z_offset) = if state.current() == e_state {
			(-200, 10.0, 1.0)
		} else {
			(-100, 5.0, 0.0)
		};
		let y_goal = (*n as isize - s.0 as isize) * distance;
		let goal = Transform::from_translation(Vec3::new(t.translation.x, y_goal as f32, 0.0));
		let distance = goal.translation - t.translation;
		t.translation += distance * ti.delta_seconds() * coefficient;
		t.translation.z = z_offset;
	}
}

pub fn menu_entry_scale(
	mut e: Query<(&MenuEntry, &mut Text, &GameState)>,
	s: Res<MenuSelected>,
	state: Res<State<GameState>>,
) {
	for (MenuEntry(n), mut text, e_state) in e.iter_mut() {
		if state.current() == e_state {
			if s.0 == *n {
				for text in &mut text.sections {
					text.style.color = Color::WHITE;
					text.style.font_size = 100.0;
				}
			} else {
				for text in &mut text.sections {
					text.style.color = Color::WHITE;
					text.style.font_size = 75.0;
				}
			}
		} else {
			for text in &mut text.sections {
				text.style.color = Color::DARK_GRAY;
				text.style.font_size = 50.0;
			}
		}
	}
}

pub struct MenuChoose(pub usize, pub GameState);

pub fn menu_entry_choose_position(
	mut e: EventWriter<MenuChoose>,
	mut s: ResMut<MenuSelected>,
	state: Res<State<GameState>>,
	mut keys: ResMut<Input<KeyCode>>,
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
		keys.reset(KeyCode::Return);
		e.send(MenuChoose(s.0, state.current().clone()));
	}
}

pub fn mk_text_entry(
	c: &mut Commands,
	index: usize,
	asset_server: &Res<AssetServer>,
	text: String,
	windows: &Res<Windows>,
	state: &Res<State<GameState>>,
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
		text_2d_size: Text2dSize {
			size: Size {
				width: windows.get_primary().unwrap().width() / 2.0,
				height: f32::MAX,
			},
		},
		..Default::default()
	})
	.insert(state.current().clone())
	.insert(MenuEntry(index));
}

pub struct GrayOut;

pub fn gray_out(mut c: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
	c.spawn_bundle(SpriteBundle {
		sprite: Sprite::new(Vec2::new(f32::MAX, f32::MAX)),
		transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.5)),
		material: materials.add(ColorMaterial::color(Color::rgba_u8(0, 0, 0, 128))),
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

pub fn mk_back_entry(
	c: &mut Commands,
	index: usize,
	asset_server: &Res<AssetServer>,
	state: &Res<State<GameState>>,
) {
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
	.insert(state.current().clone())
	.insert(MenuEntry(index));
}

pub fn back_entry_open(
	mut s: ResMut<MenuSelected>,
	mut e: EventReader<MenuChoose>,
	mut state: ResMut<State<crate::GameState>>,
	entry: Query<(&MenuEntry, &BackEntry)>,
) {
	for _ in e.iter() {
		for (MenuEntry(n), _) in entry.iter() {
			if s.0 == *n {
				s.0 = 0;
				state.pop().unwrap();
			}
		}
	}
}

pub fn cleanup_entries<const STATE: GameState>(mut c: Commands, e: Query<(Entity, &GameState)>) {
	for (e, s) in e.iter() {
		if &STATE == s {
			c.entity(e).despawn()
		}
	}
}
