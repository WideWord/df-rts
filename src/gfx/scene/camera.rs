use cgmath::{Matrix4, One};

use ::math::{Spatial, Real};

#[derive(Clone, Copy)]
pub struct Camera {
	pub spatial: Spatial,
	pub projection: Matrix4<Real>,
}

impl Camera {

	pub fn new() -> Self {
		Camera {
			spatial: Spatial::identity(),
			projection: Matrix4::one(),
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

	pub fn calculate(camera: &Camera) -> Self {
		let view = Matrix4::from(camera.spatial.rotation.conjugate()) * Matrix4::from_translation(-camera.spatial.position);
		let view_projection = camera.projection * view;

		RenderingPrecalculatedCamera {
			spatial: camera.spatial,
			projection: camera.projection,
			view: view,
			view_projection: view_projection,
		}
	}

}
