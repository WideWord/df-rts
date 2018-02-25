use glium::glutin::{Event, DeviceEvent, ElementState, WindowEvent, MouseButton};
use enum_map::EnumMap;

use ::math::*;

#[derive(PartialEq, Eq, Clone, Copy, EnumMap)]
pub enum Key {
	Forward,
	Backward,
	Left,
	Right,
	LookAround,
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
	delta_mouse: Vector2,
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

	pub fn consume_event(&mut self, event: Event) {
		match event {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::KeyboardInput { input, .. } => if let Some(key) = self.key_from_scancode(input.scancode) {
					match input.state {
						ElementState::Pressed => if self.key_states[key] == KeyState::Up || self.key_states[key] == KeyState::Released {
							self.key_states[key] = KeyState::Pressed;
						}
						ElementState::Released => if self.key_states[key] == KeyState::Down || self.key_states[key] == KeyState::Pressed {
							self.key_states[key] = KeyState::Released;
						}
					}
				},
				WindowEvent::MouseInput { button, state, .. } => if let Some(key) = self.key_from_mouse_button(button) {
					match state {
						ElementState::Pressed => if self.key_states[key] == KeyState::Up || self.key_states[key] == KeyState::Released {
							self.key_states[key] = KeyState::Pressed;
						}
						ElementState::Released => if self.key_states[key] == KeyState::Down || self.key_states[key] == KeyState::Pressed {
							self.key_states[key] = KeyState::Released;
						}
					}
				}
				_ => (),
			},
			Event::DeviceEvent { event, .. } => match event {
				DeviceEvent::MouseMotion { delta } => {
					self.delta_mouse += vec2(delta.0 as Real, delta.1 as Real);
				},
				_ => (),
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

	pub fn delta_mouse(&self) -> Vector2 {
		self.delta_mouse
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

	fn key_from_mouse_button(&self, mouse_button: MouseButton) -> Option<Key> {
		match mouse_button {
			MouseButton::Right => Some(Key::LookAround),
			_ => None,
		}
	}

}
