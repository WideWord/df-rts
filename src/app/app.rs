use glium::glutin::{EventsLoop, Event, WindowEvent};
use cgmath::{vec3};

use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;

use ::gfx::Renderer;
use ::gfx::scene::Scene as GraphicsScene;
use ::gfx::{Mesh, MeshVertex, Material};
use ::gfx::scene::MeshInstance;
use ::math::Spatial;
use ::assets::{AssetRef, load_texture, load_mesh};

pub struct App {
	events_loop: Rc<RefCell<EventsLoop>>,
	running: bool,

	renderer: Rc<Renderer>,

	graphics_scene: Option<Rc<RefCell<GraphicsScene>>>,
}

impl App {

	pub fn new() -> Self {

		let events_loop = Rc::new(RefCell::new(EventsLoop::new()));

		let renderer = Rc::new(Renderer::new(&events_loop.borrow_mut()));

		App {
			events_loop: events_loop,
			running: true,

			renderer: renderer,

			graphics_scene: Some(Rc::new(RefCell::new(GraphicsScene::new()))),
		}	
	}

	pub fn run(&mut self) {

		{
			let texture = load_texture(self.renderer.get_display(), PathBuf::from("data/sand.jpg").as_path());

			let material = AssetRef::from(Material {
				albedo: texture,
			});

			let mesh = load_mesh(self.renderer.get_display(), PathBuf::from("data/monkey.dae").as_path(), material);

			let instance = MeshInstance {
				spatial: Spatial::identity(),
				is_static: false,
				mesh: mesh,
			};

			let scene = self.get_graphics_scene().clone().unwrap();
			scene.borrow_mut().add_mesh_instance(instance);

			let mut camera_pos = Spatial::identity();
			camera_pos.position = vec3(0.0, 0.0, 10.0);
			scene.borrow_mut().move_camera(camera_pos);
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
