use glium::{Program, Display, Frame, Surface};
use glium::uniforms;

use super::scene::{Entity, RenderingPrecalculatedCamera};

pub struct EntityRenderer {
	shader: Program,
}

impl EntityRenderer {

	pub fn new(display: &Display) -> Self {
		let vertex_shader_src = r#"
#version 140

			in vec3 position;

			void main() {
				gl_Position = vec4(position, 1.0);
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

		EntityRenderer {
			shader: shader,
		}
	}

	pub fn render(&self, target: &mut Frame, _camera: &RenderingPrecalculatedCamera, entity: &Entity) {
		let mesh = entity.mesh.borrow();
		let (vertex_buffer, index_buffer) = mesh.get_buffers();

		target.draw(vertex_buffer, index_buffer, &self.shader, &uniforms::EmptyUniforms,
            &Default::default()).unwrap();

	}

}
