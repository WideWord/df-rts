use cgmath::{perspective, ortho, Deg};

use ::math::*;

#[derive(Clone, Copy)]
pub enum CameraProjection {
	Ortho,
	Perspective,
}

#[derive(Clone, Copy)]
pub struct Camera {
	pub spatial: Spatial,
	pub z_near: Real,
	pub z_far: Real,
	pub fov_y: Deg<Real>,
	pub size_y: Real,
	pub projection: CameraProjection,
}

impl Camera {

	pub fn ortho(size_y: Real) -> Self {
		Camera {
			projection: CameraProjection::Ortho,
			size_y: size_y,
			.. Default::default()
		}
	}

}

impl Default for Camera {
	fn default() -> Self {
		Camera {
			spatial: Spatial::identity(),
			z_near: 1.0,
			z_far: 1000.0,
			fov_y: Deg(65.0),
			size_y: 1.0,
			projection: CameraProjection::Perspective,
		}
	}
}

#[derive(Copy, Clone)]
pub struct CameraRenderParameters {
	pub spatial: Spatial,
	pub projection_matrix: Matrix4,
	pub view_matrix: Matrix4,
	pub view_projection_matrix: Matrix4,
	pub inverse_view_matrix: Matrix4,
	pub inverse_projection_matrix: Matrix4,
	pub frustum: Frustum,
}

impl CameraRenderParameters {

	pub fn new(camera: &Camera, frame_size: (u32, u32)) -> Self {

		let aspect_ratio = (frame_size.0 as Real) / (frame_size.1 as Real);

		let projection_matrix = {
			let mut matrix = match camera.projection {
				CameraProjection::Ortho => {
					let ys = camera.size_y * 0.5;
					let xs = ys * aspect_ratio * 0.5;
					ortho(xs, -xs, -ys, ys, camera.z_near, camera.z_far)
				}
				CameraProjection::Perspective => perspective(camera.fov_y, (frame_size.0 as Real) / (frame_size.1 as Real), camera.z_near, camera.z_far)
			};

			// By default cgmath builds left-handed projection matrix
			// convert it to right-handed
			matrix[2][2] = -matrix[2][2];
			matrix[2][3] = -matrix[2][3];

			matrix
		};

		let view_matrix = camera.spatial.inverse_transform_matrix();
		let view_projection_matrix = projection_matrix * view_matrix;

		let view_space_frustum = match camera.projection {
			CameraProjection::Perspective => {
				let fov_y_tan = camera.fov_y.tan();
				let v_far = fov_y_tan * camera.z_far;
				let h_far = v_far * aspect_ratio;
				let v_near = fov_y_tan * camera.z_near;	
				let h_near = v_near * aspect_ratio;

				let left = Plane::from_points(vec3(-h_near, -v_near, camera.z_near), vec3(-h_far, v_far, camera.z_far), vec3(-h_near, v_near, camera.z_near));
				let right = Plane::from_points(vec3(h_near, -v_near, camera.z_near), vec3(h_near, v_near, camera.z_near), vec3(h_far, v_far, camera.z_far));
				let top = Plane::from_points(vec3(-h_near, v_near, camera.z_near), vec3(h_near, v_near, camera.z_near), vec3(h_far, v_far, camera.z_far));
				let bottom = Plane::from_points(vec3(-h_near, -v_near, camera.z_near), vec3(h_far, -v_far, camera.z_far), vec3(h_near, -v_near, camera.z_near));
				let near = Plane::from_points(vec3(-h_near, -v_near, camera.z_near), vec3(h_near, -v_near, camera.z_near), vec3(h_near, v_near, camera.z_near));
				let far = Plane::from_points(vec3(-h_near, -v_near, camera.z_near), vec3(h_near, -v_near, camera.z_near), vec3(h_near, v_near, camera.z_near));
				
				Frustum {
					left: left,
					right: right,
					top: top,
					bottom: bottom,
					near: near,
					far: far,
				}
			}
			CameraProjection::Ortho => {
				let v = camera.size_y;
				let h = v * aspect_ratio;

				let left = Plane::from_points(vec3(-h, -v, camera.z_near), vec3(-h, v, camera.z_far), vec3(-h, v, camera.z_near));
				let right = Plane::from_points(vec3(h, -v, camera.z_near), vec3(h, v, camera.z_near), vec3(h, v, camera.z_far));
				let top = Plane::from_points(vec3(-h, v, camera.z_near), vec3(h, v, camera.z_near), vec3(h, v, camera.z_far));
				let bottom = Plane::from_points(vec3(-h, -v, camera.z_near), vec3(h, -v, camera.z_far), vec3(h, -v, camera.z_near));
				let near = Plane::from_points(vec3(-h, -v, camera.z_near), vec3(h, -v, camera.z_near), vec3(h, v, camera.z_near));
				let far = Plane::from_points(vec3(-h, -v, camera.z_near), vec3(h, -v, camera.z_near), vec3(h, v, camera.z_near));
				
				Frustum {
					left: left,
					right: right,
					top: top,
					bottom: bottom,
					near: near,
					far: far,
				}
			}
		};

		let world_space_frustum = view_matrix * view_space_frustum;

		CameraRenderParameters {
			spatial: camera.spatial,
			projection_matrix: projection_matrix,
			view_matrix: view_matrix,
			view_projection_matrix: view_projection_matrix,
			inverse_view_matrix: view_matrix.inverse_transform().unwrap(),
			inverse_projection_matrix: projection_matrix.inverse_transform().unwrap(),
			frustum: world_space_frustum,
		}
	}

}
