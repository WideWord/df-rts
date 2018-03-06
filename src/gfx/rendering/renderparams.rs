use glium::DrawParameters;

use ::gfx::scene::{CameraRenderParams};

pub enum RenderPassType {
	GBuffer,
	ShadowMap,
}

pub struct RenderParams<'a> {
	pub camera: CameraRenderParams,
	pub draw_parameters: DrawParameters<'a>,
	pub pass_type: RenderPassType,
}
