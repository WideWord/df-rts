extern crate glium;

use glium::glutin::{EventsLoop, Event, WindowEvent};
use ::gfx::Renderer;
use std::rc::Rc;
use std::cell::RefCell;

pub struct MainLoop {
	events_loop: Rc<RefCell<EventsLoop>>,
	renderer: Renderer,
	running: bool,
}

impl MainLoop {

	pub fn new() -> Self {

		let events_loop = Rc::new(RefCell::new(EventsLoop::new()));

		let renderer = Renderer::new(&events_loop.borrow_mut());

		return MainLoop {
			events_loop: events_loop,
			renderer: renderer,
			running: true
		}	
	}

	pub fn run(&mut self) {
		while self.running {
			self.process_events();
			self.renderer.render();
		}
	}

	pub fn quit(&mut self) {
		self.running = false
	}

	fn process_events(&mut self) {
		let events_loop = self.events_loop.clone();

		events_loop.borrow_mut().poll_events(|event| {
			self.process_event(event);
		});
	}

	fn process_event(&mut self, event: Event) {
		match event {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::Closed => self.quit(),
				_ => (),
			},
			_ => (),
		}
	}

}
