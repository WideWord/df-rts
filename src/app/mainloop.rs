extern crate glium;

use glium::glutin::{EventsLoop, WindowBuilder, ContextBuilder, Event, WindowEvent};
use glium::{Surface, Display};
use std::rc::Rc;
use std::cell::RefCell;

pub struct MainLoop {
	events_loop: Rc<RefCell<EventsLoop>>,
	display: glium::Display,
	running: bool,
}

impl MainLoop {

	pub fn new() -> Self {

		let events_loop = Rc::new(RefCell::new(EventsLoop::new()));

		let window = WindowBuilder::new();

		let context = ContextBuilder::new();

		let display = Display::new(window, context, &events_loop.borrow_mut()).unwrap();

		return MainLoop {
			events_loop: events_loop,
			display: display,
			running: true
		}	
	}

	pub fn run(&mut self) {
		while self.running {

			let events_loop = self.events_loop.clone();

			events_loop.borrow_mut().poll_events(|ev| {
				match ev {
					Event::WindowEvent { event, .. } => match event {
						WindowEvent::Closed => self.quit(),
						_ => (),
					},
					_ => (),
				}
			});

			let mut target = self.display.draw();

			target.clear_color(0.0, 0.0, 1.0, 1.0);
			target.finish().unwrap();
		}
	}

	pub fn quit(&mut self) {
		self.running = false
	}

}
