use glium::glutin::{KeyboardInput, DeviceEvent, ElementState};
use enum_map::EnumMap;
use cgmath::{Vector2, vec2};

use ::math::Real;

#[derive(PartialEq, Eq, Clone, Copy, EnumMap)]
pub enum Key {
	Forward,
	Backward,
	Left,
	Right,
}

#[derive(PartialEq, Eq, Clone, Copy, EnumMap)]
pub enum KeyState {
	Up,
	Pressed,
	Down,
	Released,
}

impl Default for KeyState {
	fn default() -> Self {
		KeyState::Up
	}
}

pub struct Input {
	key_states: EnumMap<Key, KeyState>,
	delta_mouse: Vector2<Real>,
}

impl Input {

	pub fn new() -> Self {
		Input {
			key_states: EnumMap::default(),
			delta_mouse: vec2(0.0, 0.0),
		}
	}

	pub fn new_frame(&mut self) {
		for (_, state) in &mut self.key_states {
			match *state {
				KeyState::Pressed => *state = KeyState::Down,
				KeyState::Released => *state = KeyState::Up,
				_ => (),
			}
		}
		self.delta_mouse = vec2(0.0, 0.0);
	}

	pub fn consume_keyboard(&mut self, event: KeyboardInput) {
		if let Some(key) = self.key_from_scancode(event.scancode) {
			match event.state {
				ElementState::Pressed => if self.key_states[key] == KeyState::Up || self.key_states[key] == KeyState::Released {
					self.key_states[key] = KeyState::Pressed;
				}
				ElementState::Released => if self.key_states[key] == KeyState::Down || self.key_states[key] == KeyState::Pressed {
					self.key_states[key] = KeyState::Released;
				}
			}
		}
	}

	pub fn consume_device(&mut self, event: DeviceEvent) {
		match event {
			DeviceEvent::MouseMotion { delta } => {
				self.delta_mouse += vec2(delta.0 as Real, delta.1 as Real);
			},
			_ => (),
		}
	}

	pub fn is_key_down(&self, key: Key) -> bool {
		match self.key_states[key] {
			KeyState::Pressed | KeyState::Down => true,
			_ => false,
		}
	}

	fn key_from_scancode(&self, scancode: u32) -> Option<Key> {
		match scancode {
			13 => Some(Key::Forward),
			0 => Some(Key::Left),
			1 => Some(Key::Backward),
			2 => Some(Key::Right),
			_ => None,
		}
	}

}
