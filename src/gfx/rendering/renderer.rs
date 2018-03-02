use glium::glutin::{EventsLoop, WindowBuilder, ContextBuilder};
use glium::{Surface, Display, Rect, DrawParameters};

use ::gfx::scene::{Scene, CameraRenderParameters};
use ::gfx::rendering::{MeshRenderer, TerrainRenderer, GBuffer, RenderParameters, RenderPassType};
use ::gfx::lighting::SunRenderer;

pub struct Renderer {
	display: Display,

	mesh_renderer: MeshRenderer,
	terrain_renderer: TerrainRenderer,

	g_buffer: GBuffer,

	sun_renderer: SunRenderer,
}

impl Renderer {

	pub fn new(events_loop: &EventsLoop) -> Self {

		let window = WindowBuilder::new();

		let context = ContextBuilder::new();

		let display = Display::new(window, context, events_loop).unwrap();

		let mesh_renderer = MeshRenderer::new(&display);
		let terrain_renderer = TerrainRenderer::new(&display);

		let sun_renderer = SunRenderer::new(&display);

		let g_buffer = GBuffer::new(&display, (1024 * 2, 768 * 2));

		Renderer {
			display: display,
			mesh_renderer: mesh_renderer,
			terrain_renderer: terrain_renderer,
			g_buffer: g_buffer,
			sun_renderer: sun_renderer,
		}
	}

	pub fn get_display(&self) -> &Display {
		&self.display
	}

	pub fn render(&self, scene: &Scene) {		

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
			let mut target = self.g_buffer.framebuffer(&self.display);
			target.clear_color(0.0, 0.0, 0.0, 1.0);
			target.clear_depth(1.0);

			let camera = CameraRenderParameters::new(scene.camera(), viewport);

			let render_parameters = RenderParameters {
				camera: camera,
				draw_parameters: draw_parameters.clone(),
				pass_type: RenderPassType::GBuffer,
			};

			self.draw_scene(&mut target, &render_parameters, scene);
		}

 		{
			let mut target = self.display.draw();
			target.clear_color(0.0, 0.0, 0.0, 1.0);

			if let Some(ref sun) = scene.sun {
				self.sun_renderer.draw_sun_lighting(&mut target, &draw_parameters, &self.g_buffer, sun);
			}

			target.finish().unwrap();
		}

	}

	fn draw_scene<Target: Surface>(&self, target: &mut Target, render_parameters: &RenderParameters, scene: &Scene) {
		for entity_ref in scene.get_mesh_instances() {
			self.mesh_renderer.draw_mesh_instance(target, &render_parameters, &entity_ref.0);
		}

		if let Some(ref terrain) = scene.terrain {
			self.terrain_renderer.draw_terrain(target, &render_parameters, &terrain.asset.borrow());
		}
	}

}
