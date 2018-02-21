use cgmath::{Matrix4, One};

use ::math::{Spatial, Real};

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
