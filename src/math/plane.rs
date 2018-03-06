use cgmath::prelude::*;

use ::math::{Vector3, Matrix4, Real, dot, vec3};

#[derive(Copy, Clone)]
pub struct Plane {
	pub normal: Vector3,
	pub d: Real,
}

impl Plane {

	pub fn from_points(a: Vector3, b: Vector3, c: Vector3) -> Self {
		let normal = (c - a).cross(b - a).normalize();
		Plane {
			normal: normal,
			d: dot(a, normal),
		}
	}

}

impl ::std::ops::Mul<Plane> for Matrix4 {
	type Output = Plane;

	fn mul(self, plane: Plane) -> Plane {
		let v = self.transpose() * plane.normal.extend(plane.d);
		Plane {
			normal: vec3(v.x, v.y, v.z),
			d: v.w,
		}
	}

}
