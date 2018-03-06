 use ::math::{Plane, AABB3, vec3, Matrix4};

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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum IntersectionTestResult {
	Inside,
	Outside,
	Intersect,
}

pub fn intersect_frustum_aabb(frustum: &Frustum, aabb: &AABB3) -> IntersectionTestResult {
	use self::IntersectionTestResult::*;
	let mut result = Inside;

	for plane in [&frustum.near, &frustum.far, &frustum.left, &frustum.right, &frustum.top, &frustum.bottom].iter() {
		let mut n = vec3(0.0, 0.0, 0.0);
		let mut p = vec3(0.0, 0.0, 0.0);

		// X axis 
		if plane.normal.x > 0.0 { 
			n.x = aabb.min.x; 
			p.x = aabb.max.x; 
		} else { 
			n.x = aabb.max.x; 
			p.x = aabb.min.x; 
		} 
		// Y axis 
		if plane.normal.y > 0.0 { 
			n.y = aabb.min.y; 
			p.y = aabb.max.y; 
		} else { 
			n.y = aabb.max.y; 
			p.y = aabb.min.y; 
		} 
		// Z axis 
		if plane.normal.z > 0.0 { 
			n.z = aabb.min.z; 
			p.z = aabb.max.z; 
		} else { 
			n.z = aabb.max.z; 
			p.z = aabb.min.z; 
		}

		if plane.oriented_distance(p) < 0.0 { 
			println!("considered outside");
        	return Outside;
     	} else if plane.oriented_distance(n) <= 0.0 {
     		println!("considered intersect");
     		result = Intersect;
     	}
	}

	return result;
}

