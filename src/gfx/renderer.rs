use glium::glutin::{EventsLoop, WindowBuilder, ContextBuilder};
use glium::{Surface, Display, Rect, DrawParameters};

use std::rc::Rc;
use std::cell::RefCell;

use super::scene::{Scene, RenderingPrecalculatedCamera};
use super::{MeshRenderer, GBuffer};
use super::lighting::LightRenderer;

pub struct Renderer {
	display: Display,

	mesh_renderer: MeshRenderer,

	g_buffer: GBuffer,

	light_renderer: LightRenderer,
}

impl Renderer {

	pub fn new(events_loop: &EventsLoop) -> Self {

		let window = WindowBuilder::new();

		let context = ContextBuilder::new();

		let display = Display::new(window, context, events_loop).unwrap();

		let mesh_renderer = MeshRenderer::new(&display);

		let light_renderer = LightRenderer::new(&display);

		let g_buffer = GBuffer::new(&display, (1024 * 2, 768 * 2));

		Renderer {
			display: display,
			mesh_renderer: mesh_renderer,
			g_buffer: g_buffer,
			light_renderer: light_renderer,
		}
	}

	pub fn get_display(&self) -> &Display {
		&self.display
	}

	pub fn render(&self, scene: &Rc<RefCell<Scene>>) {		

		let viewport = (1024 * 2, 768 * 2);

		let draw_parameters = DrawParameters {
			viewport: Some(Rect {
				left: 0,
				bottom: 0,
				height: viewport.1,
				width: viewport.0,
			}),
			.. Default::default()
		};

		{
			let mut target = self.g_buffer.get_framebuffer(&self.display);
			target.clear_color(0.0, 0.0, 0.0, 1.0);

			let precalculated_camera = RenderingPrecalculatedCamera::calculate(scene.borrow().get_camera(), viewport);

			

			for entity_ref in scene.borrow().get_mesh_instances() {
				self.mesh_renderer.render(&mut target, &draw_parameters, &precalculated_camera, &entity_ref.0);
			}

		}
 		{
			let mut target = self.display.draw();
			target.clear_color(0.0, 1.0, 1.0, 1.0);

			self.light_renderer.render(&mut target, &draw_parameters, &self.g_buffer);

			target.finish().unwrap();
		}
	}

}
