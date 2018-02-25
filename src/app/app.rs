use glium::glutin::{EventsLoop};

use std::rc::Rc;
use std::cell::RefCell;
use std::path::PathBuf;
use std::time::{SystemTime};

use ::gfx::rendering::Renderer;
use ::gfx::scene::Scene as GraphicsScene;
use ::gfx::resources::{Material};
use ::gfx::scene::MeshInstance;
use ::math::*;
use ::assets::Asset;
use ::assets::util::*;
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

				let material = Asset::asset(Material {
					albedo: texture,
				});

				let mesh = load_mesh(self.renderer.get_display(), PathBuf::from("data/monkey.dae").as_path(), material.clone());

				let instance = MeshInstance {
					spatial: Spatial::identity(),
					is_static: false,
					mesh: mesh,
				};

				scene.borrow_mut().add_mesh_instance(instance);
			
				let map = load_texture(self.renderer.get_display(), PathBuf::from("data/terrain.png").as_path());

				let terrain = Asset::asset(Terrain::new(map));

				terrain.asset.borrow_mut().materials.push(material.clone());

				scene.borrow_mut().set_terrain(terrain);
			
			
		}
		
		while self.running {


			self.process_events();

			let delta_duration = self.last_frame_time.elapsed().unwrap();
			self.delta_time = (delta_duration.as_secs() as f32) + (delta_duration.subsec_nanos() as f32) * 0.000000001;
			self.last_frame_time = SystemTime::now();

			{
				if self.input.is_key_down(input::Key::LookAround) {
					let scene_ref = self.graphics_scene.clone().unwrap();
					let mut scene = scene_ref.borrow_mut();
					let camera = scene.camera_mut();
					let delta_mouse = self.input.delta_mouse();

					camera.spatial.rotation = Quaternion::from_angle_y(Rad(-delta_mouse.x * 0.01)) * camera.spatial.rotation * Quaternion::from_angle_x(Rad(-delta_mouse.y * 0.01));
				}

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

	pub fn graphics_scene(&self) -> Option<Rc<RefCell<GraphicsScene>>> {
		self.graphics_scene.clone()
	}

	fn process_events(&mut self) {

		self.input.new_frame();

		let events_loop = self.events_loop.clone();

		events_loop.borrow_mut().poll_events(|event| {
			self.input.consume_event(event);
		});
	}


	fn render_scene(&self) {
		if let Some(ref scene) = self.graphics_scene {
			self.renderer.render(&scene.borrow())
		}
	}

}
