use glium::{Display, Program, Surface, VertexBuffer, IndexBuffer, DrawParameters};
use glium::index::PrimitiveType;

use ::gfx::GBuffer;

#[derive(Copy, Clone)]
struct QuadVertex {
	position: [f32; 2],
}

implement_vertex!(QuadVertex, position);

pub struct LightRenderer {
	shader: Program,
	vertex_buffer: VertexBuffer<QuadVertex>,
	index_buffer: IndexBuffer<u16>,
}

impl LightRenderer {

	pub fn new(display: &Display) -> Self {

		let vertex_shader_src = r#"
			#version 140

			in vec2 position;

			out vec2 v_position;

			void main() {
				gl_Position = vec4(position, 0.0, 1.0);
				v_position = (position + vec2(1.0, 1.0)) * 0.5;
			}
		"#;

		let fragment_shader_src = r#"
			#version 140

			in vec2 v_position;

			uniform sampler2D s_albedo;
			uniform sampler2D s_normal;

			out vec4 color;

			void main() {
				vec4 t_albedo = texture(s_albedo, v_position);
				vec4 t_normal = texture(s_normal, v_position);

				vec3 normal = t_normal.xyz * 2 - vec3(1, 1, 1);
				vec3 albedo = t_albedo.rgb;

				float lighting = dot(normalize(normal.xyz), -normalize(vec3(0.3, -1, -0.3)));

				color = vec4(lighting * albedo.rgb, 1);
			}
		"#;

		let shader = Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

		let verticies = [
			QuadVertex { position: [-1.0, -1.0] },
			QuadVertex { position: [-1.0,  1.0] },
			QuadVertex { position: [ 1.0,  1.0] },
			QuadVertex { position: [ 1.0, -1.0] },
		];

		let indicies: [u16; 6] = [0, 1, 2, 0, 2, 3];

		let vertex_buffer = VertexBuffer::new(display, &verticies).unwrap();

		let index_buffer = IndexBuffer::new(
			display,
			PrimitiveType::TrianglesList,
            &indicies
        ).unwrap();


		LightRenderer {
			shader: shader,
			vertex_buffer: vertex_buffer,
			index_buffer: index_buffer,
		}
	}

	pub fn render<F: Surface>(&self, target: &mut F, draw_parameters: &DrawParameters, g_buffer: &GBuffer) {

		let uniforms = uniform! {
			s_albedo: g_buffer.albedo_texture(),
			s_normal: g_buffer.normal_texture(),
		};

		target.draw(&self.vertex_buffer, &self.index_buffer, &self.shader, &uniforms, draw_parameters).unwrap();
	}

}
