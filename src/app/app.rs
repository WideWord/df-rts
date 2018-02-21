use glium::glutin::{EventsLoop, Event, WindowEvent};

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use ::gfx::Renderer;
use ::assets::AssetsManager;
use ::gfx::scene::Scene as GraphicsScene;

pub struct App {
	events_loop: Rc<RefCell<EventsLoop>>,
	running: bool,

	renderer: Rc<Renderer>,
	assets_manager: Rc<AssetsManager>,

	graphics_scene: RefCell<Option<Rc<RefCell<GraphicsScene>>>>,
}

impl App {

	pub fn new() -> Self {

		let events_loop = Rc::new(RefCell::new(EventsLoop::new()));

		let renderer = Rc::new(Renderer::new(&events_loop.borrow_mut()));

		let assets_manager = Rc::new(AssetsManager::new(renderer.clone()));

		App {
			events_loop: events_loop,
			running: true,

			renderer: renderer,
			assets_manager: assets_manager,

			graphics_scene: RefCell::new(Some(Rc::new(RefCell::new(GraphicsScene::new())))),
		}	
	}

	pub fn run(&mut self) {
		while self.running {
			self.process_events();

			self.render_scene();
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

	fn render_scene(&self) {
		if let &Some(ref scene) = self.graphics_scene.borrow().deref() {
			self.renderer.render(scene.clone())
		}
	}

}