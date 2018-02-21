use glium::glutin::{EventsLoop, Event, WindowEvent};

use std::rc::Rc;
use std::cell::RefCell;

use ::gfx::Renderer;
use ::assets::AssetsManager;
use ::gfx::scene::Scene as GraphicsScene;
use ::gfx::{Mesh, MeshVertex};
use ::gfx::scene::MeshInstance;
use ::math::Spatial;

pub struct App {
	events_loop: Rc<RefCell<EventsLoop>>,
	running: bool,

	renderer: Rc<Renderer>,
	assets_manager: Rc<AssetsManager>,

	graphics_scene: Option<Rc<RefCell<GraphicsScene>>>,
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

			graphics_scene: Some(Rc::new(RefCell::new(GraphicsScene::new()))),
		}	
	}

	pub fn run(&mut self) {

		{
			let vertex1 = MeshVertex { position: [-0.5, -0.5, 0.0], normal: [0.0, 0.0, 0.0] };
			let vertex2 = MeshVertex { position: [ 0.0,  0.5, 0.0], normal: [0.0, 0.0, 0.0] };
			let vertex3 = MeshVertex { position: [ 0.5, -0.25, 0.0], normal: [0.0, 0.0, 0.0] };
			let shape = vec![vertex1, vertex2, vertex3];

			let index: [u16; 3] = [0, 1, 2];

			let mesh = Rc::new(RefCell::new(Mesh::new(self.renderer.get_display(), &shape, &index)));

			let instance = MeshInstance {
				spatial: Spatial::identity(),
				is_static: false,
				mesh: mesh,
			};

			let scene = self.get_graphics_scene().clone().unwrap();
			scene.borrow_mut().add_mesh_instance(instance);
		}
		
		while self.running {
			self.process_events();

			self.render_scene();
		}
	}

	pub fn quit(&mut self) {
		self.running = false
	}

	pub fn set_graphics_scene(&mut self, scene: Option<Rc<RefCell<GraphicsScene>>>) {
		self.graphics_scene = scene;
	}

	pub fn get_graphics_scene(&self) -> Option<Rc<RefCell<GraphicsScene>>> {
		self.graphics_scene.clone()
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
		if let Some(ref scene) = self.graphics_scene {
			self.renderer.render(&scene.clone())
		}
	}

}
