use glium::glutin::{EventsLoop, WindowBuilder, ContextBuilder};
use glium::{Surface, Display};

use std::rc::Rc;
use std::cell::RefCell;

use super::scene::{Scene, RenderingPrecalculatedCamera};
use super::MeshRenderer;

pub struct Renderer {
	display: Display,
	mesh_renderer: MeshRenderer,
}

impl Renderer {

	pub fn new(events_loop: &EventsLoop) -> Self {

		let window = WindowBuilder::new();

		let context = ContextBuilder::new();

		let display = Display::new(window, context, events_loop).unwrap();

		let mesh_renderer = MeshRenderer::new(&display);

		Renderer {
			display: display,
			mesh_renderer: mesh_renderer,
		}
	}

	pub fn get_display(&self) -> &Display {
		&self.display
	}

	pub fn render(&self, scene: &Rc<RefCell<Scene>>) {
		let mut target = self.display.draw();
		target.clear_color(0.0, 0.0, 1.0, 1.0);

		let precalculated_camera = RenderingPrecalculatedCamera::calculate(scene.borrow().get_camera());

		for entity_ref in scene.borrow().get_mesh_instances() {
			self.mesh_renderer.render(&mut target, &precalculated_camera, &entity_ref.0);
		}

		target.finish().unwrap();
	}

}
