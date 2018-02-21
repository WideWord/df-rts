use cgmath::{Vector3, Quaternion, Zero, One};

pub type Real = f32;

#[derive(Clone, Copy)]
pub struct Spatial {
	pub position: Vector3<Real>,
	pub rotation: Quaternion<Real>,
}

impl Spatial {

	pub fn identity() -> Self {
		Spatial {
			position: Vector3::zero(),
			rotation: Quaternion::one(),
		}
	}

}
