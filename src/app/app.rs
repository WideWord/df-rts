use glium::glutin::{EventsLoop, Event, WindowEvent};
use cgmath::{vec3};

use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;
use std::time::{SystemTime};

use ::gfx::rendering::Renderer;
use ::gfx::scene::Scene as GraphicsScene;
use ::gfx::resources::{Material};
use ::gfx::scene::MeshInstance;
use ::math::Spatial;
use ::assets::{AssetRef, load_texture, load_mesh};
use super::Input;
use super::input;
use ::terrain::Terrain;

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
			let scene = self.graphics_scene().clone().unwrap();

			let texture = load_texture(self.renderer.get_display(), PathBuf::from("data/sand.jpg").as_path());

			let material = AssetRef::from(Material {
				albedo: texture,
			});
		
			let map = load_texture(self.renderer.get_display(), PathBuf::from("data/terrain.png").as_path());

			let terrain = AssetRef::from(Terrain::new(map));

			terrain.asset.borrow_mut().materials.push(material);

			scene.borrow_mut().set_terrain(terrain);

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

				{
					let scene_ref = self.graphics_scene.clone().unwrap();
					let mut scene = scene_ref.borrow_mut();
					let camera = scene.camera_mut();
					camera.spatial.position += (camera.spatial.rotation_matrix() * tr) * self.delta_time;
				}				
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
				WindowEvent::KeyboardInput { input, .. } => self.input.consume_keyboard(input),
				_ => (),
			},
			Event::DeviceEvent { event, .. } => self.input.consume_device(event),
			_ => (),
		}
	}

	fn render_scene(&self) {
		if let Some(ref scene) = self.graphics_scene {
			self.renderer.render(&scene.clone())
		}
	}

}
