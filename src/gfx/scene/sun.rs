use glium::texture::DepthTexture2d;

use std::cell::RefCell;

use ::math::*;

pub struct SunRenderResources {
	pub shadow_map: DepthTexture2d,
	pub shadow_map_view_projection_matrix: Matrix4,
}

pub struct Sun {
	pub direction: Vector3,
	pub color: Vector3,
	pub render_resources: RefCell<Option<SunRenderResources>>,
}
