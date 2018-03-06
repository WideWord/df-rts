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
/*
	pub fn contains(&self, point: Vector3) -> bool {
	 	point.x >= self.min.x && point.x <= self.max.x &&
		point.y >= self.min.y && point.y <= self.max.y &&
		point.z >= self.min.z && point.z <= self.max.z
	}*/

}
