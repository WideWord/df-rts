use cgmath::{Matrix4, perspective, Deg};

use ::math::{Spatial, Real};

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

pub struct RenderingPrecalculatedCamera {
	pub spatial: Spatial,
	pub projection: Matrix4<Real>,
	pub view: Matrix4<Real>,
	pub view_projection: Matrix4<Real>,
}

impl RenderingPrecalculatedCamera {

	pub fn calculate(camera: &Camera, frame_size: (u32, u32)) -> Self {
		let projection = perspective(camera.fov_y, (frame_size.0 as Real) / (frame_size.1 as Real), camera.z_near, camera.z_far);
		let view = Matrix4::from(camera.spatial.rotation.conjugate()) * Matrix4::from_translation(-camera.spatial.position);
		let view_projection = projection * view;

		RenderingPrecalculatedCamera {
			spatial: camera.spatial,
			projection: projection,
			view: view,
			view_projection: view_projection,
		}
	}

}
