use cgmath;
pub use cgmath::{One, Zero};

pub type Real = f32;
pub type Vector3 = cgmath::Vector3<Real>;
pub type Quaternion = cgmath::Quaternion<Real>;
pub type Matrix3 = cgmath::Matrix3<Real>;
pub type Matrix4 = cgmath::Matrix4<Real>;
pub use cgmath::{vec2, vec3, vec4};
pub use cgmath::prelude::*;

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

pub fn matrix4_to_array(transform: Matrix4) -> [[Real; 4]; 4] {
	[
		[transform.row(0).x, transform.row(1).x, transform.row(2).x, transform.row(3).x],
		[transform.row(0).y, transform.row(1).y, transform.row(2).y, transform.row(3).y],
		[transform.row(0).z, transform.row(1).z, transform.row(2).z, transform.row(3).z],
		[transform.row(0).w, transform.row(1).w, transform.row(2).w, transform.row(3).w],
	]
}

pub fn matrix3_to_array(transform: Matrix3) -> [[Real; 3]; 3] {
	[
		[transform.row(0).x, transform.row(1).x, transform.row(2).x],
		[transform.row(0).y, transform.row(1).y, transform.row(2).y],
		[transform.row(0).z, transform.row(1).z, transform.row(2).z],
	]
}
