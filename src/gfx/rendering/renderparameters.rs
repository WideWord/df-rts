use glium::DrawParameters;

use ::gfx::scene::CameraRenderParameters;

pub struct RenderParameters<'a> {
	pub camera: CameraRenderParameters,
	pub draw_parameters: DrawParameters<'a>,
}
