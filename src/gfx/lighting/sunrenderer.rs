use glium::{Display, Program, Surface, VertexBuffer, IndexBuffer, DrawParameters};
use glium::index::PrimitiveType;

use ::gfx::rendering::GBuffer;
use ::gfx::scene::{Sun, CameraRenderParameters};
use ::math::*;

#[derive(Copy, Clone)]
struct QuadVertex {
	position: [f32; 2],
}

implement_vertex!(QuadVertex, position);

pub struct SunRenderer {
	shader: Program,
	vertex_buffer: VertexBuffer<QuadVertex>,
	index_buffer: IndexBuffer<u16>,
}

impl SunRenderer {

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

			uniform sampler2D u_albedo_metallic_map;
			uniform sampler2D u_normal_roughness_map;
			uniform sampler2D u_depth_map;
			uniform vec3 u_sun_direction;
			uniform vec3 u_sun_color;
			uniform mat4 u_inverse_projection_matrix;
			uniform mat4 u_inverse_view_matrix;

			out vec4 color;


			void main() {
				vec4 albedo_metallic = texture(u_albedo_metallic_map, v_position);
				vec4 normal_roughness = texture(u_normal_roughness_map, v_position);

				vec3 albedo = albedo_metallic.rgb;
				float metallic = albedo_metallic.a;
				vec3 normal = normal_roughness.rgb * 2 - vec3(1, 1, 1);
				float roughness = normal_roughness.a;

				float depth = texture(u_depth_map, v_position).x;
				vec4 clip_position = vec4(v_position * 2.0 - 1.0, depth * 2.0 - 1.0, 1.0);
				vec4 view_position = u_inverse_projection_matrix * clip_position;
				view_position /= view_position.w;
				vec3 position = (u_inverse_view_matrix * view_position).xyz;

				color = vec4(fract(position.rgb), 1);
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


		SunRenderer {
			shader: shader,
			vertex_buffer: vertex_buffer,
			index_buffer: index_buffer,
		}
	}

	pub fn draw_sun_lighting<F: Surface>(&self, target: &mut F, draw_parameters: &DrawParameters, g_buffer: &GBuffer, camera: &CameraRenderParameters, sun: &Sun) {

		let uniforms = uniform! {
			u_albedo_metallic_map: g_buffer.albedo_metallic_texture(),
			u_normal_roughness_map: g_buffer.normal_roughness_texture(),
			u_depth_map: g_buffer.depth_texture(),
			u_sun_direction: [sun.direction.x, sun.direction.y, sun.direction.z],
			u_sun_color: [sun.color.x, sun.color.y, sun.color.z],

			u_inverse_projection_matrix: matrix4_to_array(camera.inverse_projection_matrix),
			u_inverse_view_matrix: matrix4_to_array(camera.inverse_view_matrix),
		};

		target.draw(&self.vertex_buffer, &self.index_buffer, &self.shader, &uniforms, draw_parameters).unwrap();
	}

}
