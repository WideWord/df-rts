use cgmath::{perspective, Deg};

use ::math::*;

#[derive(Clone, Copy)]
pub struct Camera {
	pub spatial: Spatial,
	pub z_near: Real,
	pub z_far: Real,
	pub fov_y: Deg<Real>,
}

impl Camera {

	pub fn new() -> Self {
		Camera {
			spatial: Spatial::identity(),
			z_near: 1.0,
			z_far: 1000.0,
			fov_y: Deg(65.0),
		}
	}

}

pub struct CameraRenderingParameters {
	pub spatial: Spatial,
	pub projection: Matrix4,
	pub view: Matrix4,
	pub view_projection: Matrix4,
}

impl CameraRenderingParameters {

	pub fn new(camera: &Camera, frame_size: (u32, u32)) -> Self {
		let projection = perspective(camera.fov_y, (frame_size.0 as Real) / (frame_size.1 as Real), camera.z_near, camera.z_far);
		let view = camera.spatial.inverse_transform_matrix();
		let view_projection = projection * view;

		CameraRenderingParameters {
			spatial: camera.spatial,
			projection: projection,
			view: view,
			view_projection: view_projection,
		}
	}

}
