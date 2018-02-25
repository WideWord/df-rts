use glium::{Program, Display, Surface, Depth};
use glium::draw_parameters::DepthTest;

use std::ops::Deref;

use ::gfx::scene::MeshInstance;
use ::gfx::rendering::RenderParameters;
use ::math::*;

pub struct MeshRenderer {
	shader: Program,
}

impl MeshRenderer {

	pub fn new(display: &Display) -> Self {
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
				v_normal = (world_normal + vec3(1, 1, 1)) * 0.5;
			}
		"#;

		let fragment_shader_src = r#"
			#version 140

			in vec2 v_uv;
			in vec3 v_normal;

			uniform sampler2D albedo;

			out vec4 o_albedo;
			out vec4 o_normal;

			void main() {
				o_albedo = texture(albedo, v_uv);
				o_normal = vec4(v_normal, 1.0);
			}
		"#;

		let shader = Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

		MeshRenderer {
			shader: shader,
		}
	}

	pub fn render<F: Surface>(&self, target: &mut F, params: &RenderParameters, object: &MeshInstance) {
		let mesh = object.mesh.asset.borrow();
		let (vertex_buffer, index_buffer) = mesh.get_buffers();

		let model_transform = object.spatial.transform_matrix();

		let transform = params.camera.view_projection * model_transform;

		let material = mesh.material.asset.borrow();
		let albedo = material.albedo.asset.borrow();

		let uniforms = uniform! {
			transform: matrix4_to_array(transform),
			normal_transform: matrix3_to_array(object.spatial.rotation_matrix()),
			albedo: albedo.deref(),
		};

		let mut draw_parameters = params.draw_parameters.clone();

		draw_parameters.depth = Depth {
        	test: DepthTest::IfLess,
        	write: true,
        	.. Default::default()
    	};


		target.draw(vertex_buffer, index_buffer, &self.shader, &uniforms, &draw_parameters).unwrap();
	}

}
