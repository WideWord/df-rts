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
			uniform vec3 u_camera_position;
			uniform mat4 u_inverse_projection_matrix;
			uniform mat4 u_inverse_view_matrix;

			out vec4 color;

			#define PI 3.1415926

			// phong (lambertian) diffuse term
			float phong_diffuse()
			{
			    return (1.0 / PI);
			}


			// compute fresnel specular factor for given base specular and product
			// product could be NdV or VdH depending on used technique
			vec3 fresnel_factor(in vec3 f0, in float product)
			{
			    return mix(f0, vec3(1.0), pow(1.01 - product, 5.0));
			}


			float D_GGX(in float roughness, in float NdH)
			{
			    float m = roughness * roughness;
			    float m2 = m * m;
			    float d = (NdH * m2 - NdH) * NdH + 1.0;
			    return m2 / (PI * d * d);
			}

			float G_schlick(in float roughness, in float NdV, in float NdL)
			{
			    float k = roughness * roughness * 0.5;
			    float V = NdV * (1.0 - k) + k;
			    float L = NdL * (1.0 - k) + k;
			    return 0.25 / (V * L);
			}

			vec3 cooktorrance_specular(in float NdL, in float NdV, in float NdH, in vec3 specular, in float roughness, in float rim_factor)
			{
				float D = D_GGX(roughness, NdH);

			    float G = G_schlick(roughness, NdV, NdL);

			    float rim = mix(1.0 - roughness * rim_factor * 0.9, 1.0, NdV);

			    return (1.0 / rim) * specular * G * D;
			}

			void main() {
				vec4 albedo_metallic = texture(u_albedo_metallic_map, v_position);
				vec4 normal_roughness = texture(u_normal_roughness_map, v_position);

				vec3 albedo = albedo_metallic.rgb;
				float metallic = 1.0;//albedo_metallic.a;
				vec3 normal = normal_roughness.rgb * 2 - vec3(1, 1, 1);
				float roughness = 0.5;//normal_roughness.a;

				float depth = texture(u_depth_map, v_position).x;
				vec4 clip_position = vec4(v_position * 2.0 - 1.0, depth * 2.0 - 1.0, 1.0);
				vec4 view_position = u_inverse_projection_matrix * clip_position;
				view_position /= view_position.w;
				vec3 position = (u_inverse_view_matrix * view_position).xyz;

				// L - point to light
				// N - point normal
				// V - point to camera

				vec3 L = -normalize(u_sun_direction);
				vec3 N = normalize(normal);
				vec3 V = normalize(u_camera_position - position);
				vec3 H = normalize(L + V);

				// mix between metal and non-metal material, for non-metal
    			// constant base specular factor of 0.04 grey is used
    			vec3 specular = mix(vec3(0.04), albedo, metallic);
    			float NdL = max(0.0,   dot(N, L));
    			float NdV = max(0.001, dot(N, V));
    			float NdH = max(0.001, dot(N, H));
    			float HdV = max(0.001, dot(H, V));
				float LdV = max(0.001, dot(L, V));

				vec3 specfresnel = fresnel_factor(specular, HdV);
				vec3 specref = cooktorrance_specular(NdL, NdV, NdH, specfresnel, roughness, 0.0);

				specref *= vec3(NdL);

			    vec3 diffref = (vec3(1.0) - specfresnel) * phong_diffuse() * NdL;
			    
			    vec3 light_color = vec3(1.0);
			    vec3 reflected_light = specref * light_color;
			    vec3 diffuse_light = diffref * light_color;

				vec3 result = diffuse_light * mix(albedo, vec3(0.0), metallic) + reflected_light;

				color = vec4(result, 1);
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
			u_camera_position: [camera.spatial.position.x, camera.spatial.position.y, camera.spatial.position.z],

			u_inverse_projection_matrix: matrix4_to_array(camera.inverse_projection_matrix),
			u_inverse_view_matrix: matrix4_to_array(camera.inverse_view_matrix),
		};

		target.draw(&self.vertex_buffer, &self.index_buffer, &self.shader, &uniforms, draw_parameters).unwrap();
	}

}
