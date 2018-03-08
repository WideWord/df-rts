use glium::{VertexBuffer, IndexBuffer, Program, Surface, Depth, Display};
use glium::program::ProgramCreationInput;
use glium::index::PrimitiveType;
use glium::draw_parameters::{DepthTest, PolygonMode};

use std::ops::Deref;

use ::math::*;
use ::terrain::Terrain;
use ::gfx::rendering::RenderParams;


#[derive(Copy, Clone)]
struct TerrainVertex {
	position: [f32; 2],
}

implement_vertex!(TerrainVertex, position);


pub struct TessTerrainRenderer {
	shader: Program,
	vertex_buffer: VertexBuffer<TerrainVertex>,
	index_buffer: IndexBuffer<u16>,
}


impl TessTerrainRenderer {

	pub fn new(display: &Display) -> Self {

		let mut vertices = Vec::<TerrainVertex>::new();
		let mut indices = Vec::<u16>::new();

		let grid_steps: u16 = 8;
		let grid_stride = grid_steps + 1;

		for x in 0..grid_stride {
			for y in 0..grid_stride {

				let sx = (x as Real) / (grid_steps as Real);
				let sy = (y as Real) / (grid_steps as Real);
				vertices.push(TerrainVertex {
					position: [sx, sy],
				});

				if x > 0 && y > 0 {
					let a = (x - 1) * grid_stride + (y - 1);
					let b = (x - 1) * grid_stride + (y);
					let c = (x) * grid_stride + (y);
					let d = (x) * grid_stride + (y - 1);
					indices.push(a);
					indices.push(b);
					indices.push(c);
					indices.push(a);
					indices.push(c);
					indices.push(d);
				}
			}
		}

		let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();

		let index_buffer = IndexBuffer::new(
			display,
			PrimitiveType::Patches {
				vertices_per_patch: 3,
			},
            &indices
		).unwrap();


		let vertex_shader_src = r#"
			#version 410 core

			in vec2 position;
			out vec2 v_position;

			void main() {
				v_position = position;
			}
		"#;

		let tess_control_shader_src = r#"
			#version 410 core
			#define id gl_InvocationID

			layout(vertices = 3) out;

			in vec2 v_position[];

			out vec2 vt_position[];

			void main() {  
			

				const int inner = 64;
				const int outer = 64;

				if (0 == id) {
					gl_TessLevelInner[0] = inner;
					gl_TessLevelInner[1] = inner;
					gl_TessLevelOuter[0] = outer;
					gl_TessLevelOuter[1] = outer;
					gl_TessLevelOuter[2] = outer;
					gl_TessLevelOuter[3] = outer;

				}

				vt_position[id] = v_position[id];
			}
		"#;

		let tess_evaluation_shader_src = r#"
			#version 410 core

			layout(triangles, equal_spacing) in;

			uniform mat4 u_transform;
			uniform vec3 u_scale;
			uniform sampler2D u_map;

			in vec2 vt_position[];

			out vec2 vte_uv;
			out vec3 vte_normal;


			void main() {
				vec3 tc = gl_TessCoord;

				vec2 position = gl_TessCoord.x * vt_position[0] + gl_TessCoord.y * vt_position[1] + gl_TessCoord.z * vt_position[2];

				float height = texture(u_map, position).r;

				vec3 terrain_position = vec3(position.x * u_scale.x, height * u_scale.y, position.y * u_scale.z);

				vte_uv = position * u_scale.xz;
				gl_Position = u_transform * vec4(terrain_position, 1.0);
			

				float step = 1.0 / 64.0;

				float s01 = texture(u_map, position + vec2(-step, 0)).x;
    			float s21 = texture(u_map, position + vec2(step, 0)).x;
    			float s10 = texture(u_map, position + vec2(0, -step)).x;
    			float s12 = texture(u_map, position + vec2(0, step)).x;
    			vec3 va = normalize(vec3(u_scale.x / 32, (s21 - s01) * u_scale.y, 0.0));
    			vec3 vb = normalize(vec3(0.0, (s12 - s10) * u_scale.y, u_scale.z / 32));
    			vte_normal = normalize(-cross(va, vb));
			}
		"#;

		let fragment_shader_src = r#"
			#version 410 core

			in vec2 vte_uv;
			in vec3 vte_normal;

			uniform sampler2D u_albedo_map;
			uniform float u_lines_highlight;

			out vec4 o_albedo_metallic;
			out vec4 o_normal_roughness;
			out vec4 o_emission;

			void main() {
				vec3 packed_normal = (normalize(vte_normal) + vec3(1.0)) * 0.5;
				vec3 albedo = texture(u_albedo_map, vte_uv).rgb;
				o_albedo_metallic = vec4(albedo, 0.0);
				o_normal_roughness = vec4(packed_normal, 1.0);
				o_emission = vec4(1.0, 0.0, 0.0, 1.0) * u_lines_highlight;
			}
		"#;

		let shader = Program::new(display, ProgramCreationInput::SourceCode {
            vertex_shader: vertex_shader_src,
            fragment_shader: fragment_shader_src,
            geometry_shader: None,
            tessellation_control_shader: Some(tess_control_shader_src),
            tessellation_evaluation_shader: Some(tess_evaluation_shader_src),
            transform_feedback_varyings: None,
            outputs_srgb: false,
            uses_point_size: false,
        }).unwrap();


		TessTerrainRenderer {
			vertex_buffer: vertex_buffer,
			index_buffer: index_buffer,
			shader: shader,
		}
	}

	pub fn draw_terrain<Target: Surface>(&self, target: &mut Target, params: &RenderParams, terrain: &Terrain) {
		let transform = params.camera.view_projection_matrix;

		let map = terrain.map.asset.borrow();
		let material = terrain.materials[0].asset.borrow();
		let albedo = material.albedo_map.asset.borrow();

		let uniforms = uniform! {
			u_transform: matrix4_to_array(transform),
			u_scale: [terrain.scale.x, terrain.scale.y, terrain.scale.z],
			u_map: map.deref(),
			u_albedo_map: albedo.deref(),
			u_lines_highlight: 0.0 as Real,
		};

		let mut draw_parameters = params.draw_parameters.clone();

		draw_parameters.depth = Depth {
        	test: DepthTest::IfLess,
        	write: true,
        	.. Default::default()
    	};

		target.draw(&self.vertex_buffer, &self.index_buffer, &self.shader, &uniforms, &draw_parameters).unwrap();

		draw_parameters.depth = Default::default();

   		draw_parameters.polygon_mode = PolygonMode::Line;

		let uniforms = uniform! {
			u_transform: matrix4_to_array(transform),
			u_scale: [terrain.scale.x, terrain.scale.y, terrain.scale.z],
			u_map: map.deref(),
			u_albedo_map: albedo.deref(),
			u_lines_highlight: 1.0 as Real,
		};

		target.draw(&self.vertex_buffer, &self.index_buffer, &self.shader, &uniforms, &draw_parameters).unwrap();
	}

}
