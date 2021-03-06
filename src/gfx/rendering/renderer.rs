use glium::glutin::{EventsLoop, WindowBuilder, ContextBuilder};
use glium::{Surface, Display, Rect, DrawParameters, Texture2d, SyncFence};
use glium::texture::DepthTexture2d;
use glium::framebuffer::MultiOutputFrameBuffer;


use ::gfx::scene::{Scene, CameraRenderParams, Camera};
use ::gfx::rendering::{MeshRenderer, GBuffer, RenderParams, RenderPassType};
use ::gfx::lighting::SunRenderer;
use ::gfx::terrain::TessTerrainRenderer;
use ::math::*;


pub struct Renderer {
	display: Display,

	mesh_renderer: MeshRenderer,
	terrain_renderer: TessTerrainRenderer,

	g_buffer: GBuffer,

	sun_renderer: SunRenderer,
}

impl Renderer {

	pub fn new(events_loop: &EventsLoop) -> Self {

		let window = WindowBuilder::new();

		let context = ContextBuilder::new();

		let display = Display::new(window, context, events_loop).unwrap();

		let mesh_renderer = MeshRenderer::new(&display);
		let terrain_renderer = TessTerrainRenderer::new(&display);

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

		let camera = CameraRenderParams::new(scene.camera(), viewport);
		
		{
			let mut target = self.g_buffer.framebuffer(&self.display);
			target.clear_color(0.0, 0.0, 0.0, 1.0);
			target.clear_depth(1.0);

			let render_parameters = RenderParams {
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

				let shadow_map = DepthTexture2d::empty(&self.display, 1024, 1024).unwrap();

				let mut shadow_map_target = MultiOutputFrameBuffer::with_depth_buffer(&self.display, ::std::iter::empty::<(&str, &Texture2d)>(), &shadow_map).unwrap();
				shadow_map_target.clear_depth(1.0);

				let mut shadow_camera = Camera::ortho(50.0);
				shadow_camera.spatial.rotation = Quaternion::look_at(sun.direction, vec3(0.0, 1.0, 0.0));
				shadow_camera.spatial.position = -sun.direction.normalize() * 100.0;
				let shadow_camera_params = CameraRenderParams::new(&shadow_camera, (1024, 1024));

				let mut shadow_draw_parameters = draw_parameters.clone();
				shadow_draw_parameters.viewport = Some(Rect {
					left: 0,
					bottom: 0,
					height: 1024,
					width: 1024,
				});

				let shadow_render_parameters = RenderParams {
					camera: shadow_camera_params,
					draw_parameters: shadow_draw_parameters,
					pass_type: RenderPassType::ShadowMap,
				};

				self.draw_scene(&mut shadow_map_target, &shadow_render_parameters, scene);

				let fence = SyncFence::new(&self.display).unwrap();
				fence.wait();

				self.sun_renderer.draw_sun_lighting(&mut target, &draw_parameters, &self.g_buffer, &camera, sun, &shadow_map, shadow_camera_params.view_projection_matrix);
			}

			target.finish().unwrap();
		}

	}

	fn draw_scene<Target: Surface>(&self, target: &mut Target, render_parameters: &RenderParams, scene: &Scene) {
		for entity_ref in scene.get_mesh_instances() {
			self.mesh_renderer.draw_mesh_instance(target, &render_parameters, &entity_ref.0);
		}

		if let Some(ref terrain) = scene.terrain {
			self.terrain_renderer.draw_terrain(target, &render_parameters, &terrain.asset.borrow());
		}
	}

}
