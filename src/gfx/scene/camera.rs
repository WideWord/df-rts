use cgmath::{perspective, ortho, Deg};

use ::math::*;

#[derive(Clone, Copy)]
pub enum CameraProjection {
	Ortho,
	Perspective,
}

#[derive(Clone, Copy)]
pub struct Camera {
	pub spatial: Spatial,
	pub z_near: Real,
	pub z_far: Real,
	pub fov_y: Deg<Real>,
	pub size_y: Real,
	pub projection: CameraProjection,
}

impl Camera {

	pub fn ortho(size_y: Real) -> Self {
		Camera {
			projection: CameraProjection::Ortho,
			size_y: size_y,
			.. Default::default()
		}
	}

}

impl Default for Camera {
	fn default() -> Self {
		Camera {
			spatial: Spatial::identity(),
			z_near: 1.0,
			z_far: 1000.0,
			fov_y: Deg(65.0),
			size_y: 1.0,
			projection: CameraProjection::Perspective,
		}
	}
}

#[derive(Copy, Clone)]
pub struct CameraRenderParameters {
	pub spatial: Spatial,
	pub projection_matrix: Matrix4,
	pub view_matrix: Matrix4,
	pub view_projection_matrix: Matrix4,
	pub inverse_view_matrix: Matrix4,
	pub inverse_projection_matrix: Matrix4,
}

impl CameraRenderParameters {

	pub fn new(camera: &Camera, frame_size: (u32, u32)) -> Self {
		let projection = match camera.projection {
			CameraProjection::Ortho => {
				let ys = camera.size_y * 0.5;
				let xs = ys / (frame_size.1 as Real) * (frame_size.0 as Real) * 0.5;
				ortho(xs, -xs, -ys, ys, camera.z_near, camera.z_far)
			}
			CameraProjection::Perspective => perspective(camera.fov_y, (frame_size.0 as Real) / (frame_size.1 as Real), camera.z_near, camera.z_far),
		};

		let view = camera.spatial.inverse_transform_matrix();
		let view_projection = projection * view;

		CameraRenderParameters {
			spatial: camera.spatial,
			projection_matrix: projection,
			view_matrix: view,
			view_projection_matrix: view_projection,
			inverse_view_matrix: view.inverse_transform().unwrap(),
			inverse_projection_matrix: projection.inverse_transform().unwrap(),
		}
	}

}
