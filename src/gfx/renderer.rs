extern crate glium;

use glium::glutin::{EventsLoop, WindowBuilder, ContextBuilder};
use glium::{Surface, Display};


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

	pub fn render(&self) {
		let mut target = self.display.draw();

		target.clear_color(0.0, 0.0, 1.0, 1.0);
		target.finish().unwrap();
	}

}
