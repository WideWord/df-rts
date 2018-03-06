 use ::math::{Plane, AABB3, vec3, dot, Matrix4};

#[derive(Copy, Clone)]
pub struct Frustum {
	pub left: Plane,
	pub right: Plane,
	pub top: Plane,
	pub bottom: Plane,
	pub near: Plane,
	pub far: Plane,
}

impl ::std::ops::Mul<Frustum> for Matrix4 {
	type Output = Frustum;

	fn mul(self, frustum: Frustum) -> Frustum {
		Frustum {
			left: self * frustum.left,
			right: self * frustum.right,
			top: self * frustum.top,
			bottom: self * frustum.bottom,
			near: self * frustum.near,
			far: self * frustum.far,
		}
	}

}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum IntersectionTestResult {
	Inside,
	Outside,
	Intersect,
}

pub fn intersect_frustum_aabb(frustum: &Frustum, aabb: &AABB3) -> IntersectionTestResult {
	use self::IntersectionTestResult::*;
	let mut result = Inside;

	for plane in [&frustum.near, &frustum.far, &frustum.left, &frustum.right, &frustum.top, &frustum.bottom].iter() {
		let mut vmin = vec3(0.0, 0.0, 0.0);
		let mut vmax = vec3(0.0, 0.0, 0.0);

		// X axis 
		if plane.normal.x > 0.0 { 
			vmin.x = aabb.min.x; 
			vmax.x = aabb.max.x; 
		} else { 
			vmin.x = aabb.max.x; 
			vmax.x = aabb.min.x; 
		} 
		// Y axis 
		if plane.normal.y > 0.0 { 
			vmin.y = aabb.min.y; 
			vmax.y = aabb.max.y; 
		} else { 
			vmin.y = aabb.max.y; 
			vmax.y = aabb.min.y; 
		} 
		// Z axis 
		if plane.normal.z > 0.0 { 
			vmin.z = aabb.min.z; 
			vmax.z = aabb.max.z; 
		} else { 
			vmin.z = aabb.max.z; 
			vmax.z = aabb.min.z; 
		}

		if dot(plane.normal, vmin) + plane.d > 0.0 { 
        	return Outside;
     	} else if dot(plane.normal, vmax) + plane.d >= 0.0 {
     		result = Intersect;
     	}
	}

	return result;
} 
