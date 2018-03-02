use ::math::prelude::*;

#[derive(Clone, Copy)]
pub struct Spatial {
	pub position: Vector3,
	pub rotation: Quaternion,
}

impl Spatial {

	pub fn identity() -> Self {
		Spatial {
			position: Vector3::zero(),
			rotation: Quaternion::one(),
		}
	}

	pub fn transform_matrix(&self) -> Matrix4 {
		Matrix4::from_translation(self.position) * Matrix4::from(self.rotation)
	}

	pub fn inverse_transform_matrix(&self) -> Matrix4 {
		Matrix4::from(self.rotation.conjugate()) * Matrix4::from_translation(-self.position)
	}

	pub fn rotation_matrix(&self) -> Matrix3 {
		Matrix3::from(self.rotation)
	}

}


