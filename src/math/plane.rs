use ::math::{Vector3, Real};

#[derive(Copy, Clone)]
pub struct Plane {
	pub d: Real,
	pub normal: Vector3,
}

impl Plane {

	pub fn from_points(a: Vector3, b: Vector3, c: Vector3) -> Plane {
		let normal = (b - a).cross(c - a);
		Plane {
			d: 0.0,
			normal: normal,
		}
	}

}
