use cgmath;
pub use cgmath::{One, Zero};

pub type Real = f32;
pub type Vector2 = cgmath::Vector2<Real>;
pub type Vector3 = cgmath::Vector3<Real>;
pub type Quaternion = cgmath::Quaternion<Real>;
pub type Matrix3 = cgmath::Matrix3<Real>;
pub type Matrix4 = cgmath::Matrix4<Real>;
pub use cgmath::{vec2, vec3, vec4, Rad, Deg};
pub use cgmath::prelude::*;

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
