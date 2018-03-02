use glium::DrawParameters;

use ::gfx::scene::{CameraRenderParameters};

pub enum RenderPassType {
	GBuffer,
	ShadowMap,
}

pub struct RenderParameters<'a> {
	pub camera: CameraRenderParameters,
	pub draw_parameters: DrawParameters<'a>,
	pub pass_type: RenderPassType,
}
