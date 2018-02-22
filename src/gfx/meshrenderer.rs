use glium::{Program, Display, Surface, DrawParameters};
use cgmath::{Matrix4};
use cgmath::prelude::Matrix;

use std::ops::Deref;

use super::scene::{MeshInstance, RenderingPrecalculatedCamera};

pub struct MeshRenderer {
	shader: Program,
}

impl MeshRenderer {

	pub fn new(display: &Display) -> Self {
		let vertex_shader_src = r#"
			#version 140

			in vec3 position;
			in vec2 uv;

			uniform mat4 transform;

			out vec2 v_uv;

			void main() {
				gl_Position = transform * vec4(position, 1.0);
				v_uv = uv;
			}
		"#;

		let fragment_shader_src = r#"
			#version 140

			in vec2 v_uv;

			uniform sampler2D albedo;

			out vec3 o_albedo;

			void main() {
				o_albedo = texture(albedo, v_uv).rgb;
			}
		"#;

		let shader = Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

		MeshRenderer {
			shader: shader,
		}
	}

	pub fn render<F: Surface>(&self, target: &mut F, draw_parameters: &DrawParameters, camera: &RenderingPrecalculatedCamera, object: &MeshInstance) {
		let mesh = object.mesh.asset.borrow();
		let (vertex_buffer, index_buffer) = mesh.get_buffers();

		let model_transform = Matrix4::from_translation(object.spatial.position) * Matrix4::from(object.spatial.rotation);

		let transform = camera.view_projection * model_transform;

		let material = mesh.material.asset.borrow();
		let albedo = material.albedo.asset.borrow();

		let uniforms = uniform! {
			transform: [
				[transform.row(0).x, transform.row(0).y, transform.row(0).z, transform.row(0).w],
				[transform.row(1).x, transform.row(1).y, transform.row(1).z, transform.row(1).w],
				[transform.row(2).x, transform.row(2).y, transform.row(2).z, transform.row(2).w],
				[transform.row(3).x, transform.row(3).y, transform.row(3).z, transform.row(3).w],
			],
			albedo: albedo.deref(),
		};

		target.draw(vertex_buffer, index_buffer, &self.shader, &uniforms, draw_parameters).unwrap();
	}

}
