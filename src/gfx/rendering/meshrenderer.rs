use glium::{Program, Display, Surface, Depth};
use glium::draw_parameters::DepthTest;

use std::ops::Deref;

use ::gfx::scene::MeshInstance;
use ::gfx::rendering::{RenderParameters, RenderPassType};
use ::math::*;

pub struct MeshRenderer {
	shader: Program,
	shadow_map_shader: Program,
}

impl MeshRenderer {

	pub fn new(display: &Display) -> Self {
		MeshRenderer {
			shader: {
				let vertex_shader_src = r#"
					#version 140

					in vec3 position;
					in vec3 normal;
					in vec2 uv;

					uniform mat3 normal_transform;
					uniform mat4 transform;

					out vec3 v_normal;
					out vec2 v_uv;

					void main() {
						gl_Position = transform * vec4(position, 1.0);
						v_uv = uv;
						vec3 world_normal = normal_transform * normalize(normal);
						v_normal = world_normal;
					}
				"#;

				let fragment_shader_src = r#"
					#version 140

					in vec2 v_uv;
					in vec3 v_normal;

					uniform sampler2D u_albedo_map;
					uniform sampler2D u_roughness_map;
					uniform sampler2D u_metallic_map;

					out vec4 o_albedo_metallic;
					out vec4 o_normal_roughness;

					void main() {
						vec3 packed_normal = (v_normal + vec3(1.0)) * 0.5;
						o_albedo_metallic = vec4(texture(u_albedo_map, v_uv).rgb, texture(u_metallic_map, v_uv).r);
						o_normal_roughness = vec4(packed_normal, texture(u_roughness_map, v_uv).r);
					}
				"#;

				Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap()
			},

			shadow_map_shader: {
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

					void main() {}
				"#;

				Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap()
			},
		}
	}

	pub fn draw_mesh_instance<F: Surface>(&self, target: &mut F, params: &RenderParameters, object: &MeshInstance) {
		let mesh = object.mesh.asset.borrow();
		let (vertex_buffer, index_buffer) = mesh.get_buffers();

		let model_transform = object.spatial.transform_matrix();

		let transform = params.camera.view_projection_matrix * model_transform;

		let material = mesh.material.asset.borrow();
		let albedo_map = material.albedo_map.asset.borrow();
		let roughness_map = material.roughness_map.asset.borrow();
		let metallic_map = material.metallic_map.asset.borrow();

		let uniforms = uniform! {
			transform: matrix4_to_array(transform),
			normal_transform: matrix3_to_array(object.spatial.rotation_matrix()),
			u_albedo_map: albedo_map.deref(),
			u_roughness_map: roughness_map.deref(),
			u_metallic_map: metallic_map.deref(),
		};

		let mut draw_parameters = params.draw_parameters.clone();

		draw_parameters.depth = Depth {
        	test: DepthTest::IfLess,
        	write: true,
        	.. Default::default()
    	};

    	let program = match params.pass_type {
    		RenderPassType::GBuffer => &self.shader,
    		RenderPassType::ShadowMap => &self.shadow_map_shader,
    	};

		target.draw(vertex_buffer, index_buffer, program, &uniforms, &draw_parameters).unwrap();
	}

}
