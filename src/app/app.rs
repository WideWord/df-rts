use glium::glutin::{EventsLoop, Event, WindowEvent};
use cgmath::{vec3};

use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;

use ::gfx::Renderer;
use ::gfx::scene::Scene as GraphicsScene;
use ::gfx::{Material};
use ::gfx::scene::MeshInstance;
use ::math::Spatial;
use ::assets::{AssetRef, load_texture, load_mesh};
use super::Input;
use super::input;
use std::time::{Duration, SystemTime};

pub struct App {
	events_loop: Rc<RefCell<EventsLoop>>,
	running: bool,
	input: Input,
	last_frame_time: SystemTime,
	delta_time: f32,

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
			input: Input::new(),
			last_frame_time: SystemTime::now(),
			delta_time: 0.0,

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

			let scene = self.graphics_scene().clone().unwrap();
			scene.borrow_mut().add_mesh_instance(instance);
		}
		
		while self.running {


			self.process_events();

			let delta_duration = self.last_frame_time.elapsed().unwrap();
			self.delta_time = (delta_duration.as_secs() as f32) + (delta_duration.subsec_nanos() as f32) * 0.000000001;
			self.last_frame_time = SystemTime::now();

			{
				let mut tr = vec3(0.0, 0.0, 0.0);
				
				if self.input.is_key_down(input::Key::Forward) {
					tr += vec3(0.0, 0.0, -1.0);
				}
				if self.input.is_key_down(input::Key::Backward) {
					tr += vec3(0.0, 0.0, 1.0);
				}
				if self.input.is_key_down(input::Key::Left) {
					tr += vec3(-1.0, 0.0, 0.0);
				}
				if self.input.is_key_down(input::Key::Right) {
					tr += vec3(1.0, 0.0, 0.0);
				}
				let scene = self.graphics_scene.clone().unwrap();
				scene.borrow_mut().camera_mut().spatial.position += tr * self.delta_time;
			}

			self.render_scene();
		}
	}

	pub fn quit(&mut self) {
		self.running = false
	}

	pub fn set_graphics_scene(&mut self, scene: Option<Rc<RefCell<GraphicsScene>>>) {
		self.graphics_scene = scene;
	}

	pub fn graphics_scene(&self) -> Option<Rc<RefCell<GraphicsScene>>> {
		self.graphics_scene.clone()
	}

	fn process_events(&mut self) {

		self.input.new_frame();

		let events_loop = self.events_loop.clone();

		events_loop.borrow_mut().poll_events(|event| {
			self.process_event(event);
		});
	}

	fn process_event(&mut self, event: Event) {
		match event {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::Closed => self.quit(),
				WindowEvent::KeyboardInput { device_id: _, input: keyboard_event } => self.input.consume(&keyboard_event),
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
