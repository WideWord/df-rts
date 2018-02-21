use glium::glutin::{EventsLoop, WindowBuilder, ContextBuilder};
use glium::{Surface, Display};

use std::rc::Rc;
use std::cell::RefCell;

use super::scene::Scene;

pub struct Renderer {
	display: Display,
}

impl Renderer {

	pub fn new(events_loop: &EventsLoop) -> Self {

		let window = WindowBuilder::new();

		let context = ContextBuilder::new();

		let display = Display::new(window, context, events_loop).unwrap();

		Renderer {
			display: display,
		}
	}

	pub fn get_display(&self) -> &Display {
		&self.display
	}

	pub fn render(&self, scene: Rc<RefCell<Scene>>) {
		let mut target = self.display.draw();

		target.clear_color(0.0, 0.0, 1.0, 1.0);
		target.finish().unwrap();
	}

}
