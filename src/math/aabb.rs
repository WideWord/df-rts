use ::math::prelude::*;

#[derive(Copy, Clone)]
pub struct AABB3 {
	pub min: Vector3,
	pub max: Vector3,
}

impl AABB3 {

	pub fn center(&self) -> Vector3 {
		self.min + (self.max - self.min) * 0.5
	}

	#[allow(dead_code)]
	pub fn from_center_size(center: Vector3, size: Vector3) -> AABB3 {
		let half_size = size * 0.5;
		AABB3 {
			min: center - half_size,
			max: center + half_size,
		}
	}

}
