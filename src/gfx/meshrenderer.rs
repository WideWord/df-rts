use glium::{Program, Display, Frame, Surface, DrawParameters};
use cgmath::{Matrix4};
use cgmath::prelude::Matrix;

use super::scene::{MeshInstance, RenderingPrecalculatedCamera};

pub struct MeshRenderer {
	shader: Program,
}

impl MeshRenderer {

	pub fn new(display: &Display) -> Self {
		let vertex_shader_src = r#"
			#version 140

			in vec3 position;

			uniform mat4 transform;

			void main() {
				gl_Position = transform * vec4(position, 1.0);
			}
		"#;

		let fragment_shader_src = r#"
			#version 140

			out vec4 color;

			void main() {
				color = vec4(1, 0, 0, 1);
			}
		"#;

		let shader = Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

		MeshRenderer {
			shader: shader,
		}
	}

	pub fn render(&self, target: &mut Frame, draw_parameters: &DrawParameters, camera: &RenderingPrecalculatedCamera, object: &MeshInstance) {
		let mesh = object.mesh.borrow();
		let (vertex_buffer, index_buffer) = mesh.get_buffers();

		let model_transform = Matrix4::from_translation(object.spatial.position) * Matrix4::from(object.spatial.rotation);

		let transform = camera.view_projection * model_transform;
		let uniforms = uniform! {
			transform: [
				[transform.row(0).x, transform.row(0).y, transform.row(0).z, transform.row(0).w],
				[transform.row(1).x, transform.row(1).y, transform.row(1).z, transform.row(1).w],
				[transform.row(2).x, transform.row(2).y, transform.row(2).z, transform.row(2).w],
				[transform.row(3).x, transform.row(3).y, transform.row(3).z, transform.row(3).w],
			],
		};

		target.draw(vertex_buffer, index_buffer, &self.shader, &uniforms, draw_parameters).unwrap();
	}

}
