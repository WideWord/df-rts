use glium::{Program, Display, Surface, VertexBuffer, IndexBuffer, Depth};
use glium::index::PrimitiveType;
use glium::draw_parameters::{DepthTest, PolygonMode};

use std::ops::Deref;

use ::gfx::rendering::RenderParams;
use ::gfx::terrain::RenderNode;
use ::terrain::Terrain;
use ::math::*;

#[derive(Copy, Clone)]
struct TerrainVertex {
	position: [f32; 2],
	height_offset: f32,
}

implement_vertex!(TerrainVertex, position, height_offset);

pub struct TerrainRenderer {
	shader: Program,
	vertex_buffer: VertexBuffer<TerrainVertex>,
	index_buffer: IndexBuffer<u16>,
}

impl TerrainRenderer {

	pub fn new(display: &Display) -> Self {
		let vertex_shader_src = r#"
			#version 140

			in vec2 position;
			in float height_offset;

			uniform mat4 u_transform;
			uniform vec3 u_scale;
			uniform vec2 u_lod_offset;
			uniform float u_lod_scale;
			uniform sampler2D u_map;

			out vec2 v_uv;
			out vec3 v_normal;


			void main() {
				vec2 lod_position = position * u_lod_scale + u_lod_offset;
				vec4 map = texture(u_map, lod_position);
				vec3 terrain_position = vec3(lod_position.x * u_scale.x, map.r * u_scale.y + height_offset, lod_position.y * u_scale.z);
				gl_Position = u_transform * vec4(terrain_position, 1.0);
				v_uv = terrain_position.xz;

				float step = 1.0 / 64.0;

				float s01 = texture(u_map, lod_position + vec2(-step, 0)).x;
    			float s21 = texture(u_map, lod_position + vec2(step, 0)).x;
    			float s10 = texture(u_map, lod_position + vec2(0, -step)).x;
    			float s12 = texture(u_map, lod_position + vec2(0, step)).x;
    			vec3 va = normalize(vec3(u_scale.x / 32, (s21 - s01) * u_scale.y, 0.0));
    			vec3 vb = normalize(vec3(0.0, (s12 - s10) * u_scale.y, u_scale.z / 32));
    			v_normal = -cross(va, vb);
			}
		"#;

		let fragment_shader_src = r#"
			#version 140

			in vec2 v_uv;
			in vec3 v_normal;

			uniform sampler2D u_albedo_map;
			uniform float u_lines_highlight;

			out vec4 o_albedo_metallic;
			out vec4 o_normal_roughness;
			out vec4 o_emission;

			void main() {
				vec3 packed_normal = (normalize(v_normal) + vec3(1.0)) * 0.5;
				vec3 albedo = texture(u_albedo_map, v_uv).rgb;
				o_albedo_metallic = vec4(albedo, 0.0);
				o_normal_roughness = vec4(packed_normal, 1.0);
				o_emission = vec4(1.0, 0.0, 0.0, 1.0) * u_lines_highlight;
			}
		"#;

		let shader = Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

		let mut vertices = Vec::<TerrainVertex>::new();
		let mut indices = Vec::<u16>::new();

		let grid_steps: u16 = 32;
		let grid_stride = grid_steps + 1;
		for x in 0..grid_stride {
			for y in 0..grid_stride {
				let sx = (x as Real) / (grid_steps as Real);
				let sy = (y as Real) / (grid_steps as Real);
				vertices.push(TerrainVertex {
					position: [sx, sy],
					height_offset: 0.0,
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

		for i in 0..grid_stride {
			let si = (i as Real) / (grid_steps as Real);

			vertices.push(TerrainVertex {
				position: [si, 0.0],
				height_offset: -1.0,
			});

			vertices.push(TerrainVertex {
				position: [si, 1.0],
				height_offset: -1.0,
			});

			vertices.push(TerrainVertex {
				position: [0.0, si],
				height_offset: -1.0,
			});

			vertices.push(TerrainVertex {
				position: [1.0, si],
				height_offset: -1.0,
			});

			if i > 0 {
				{
					let a = (i - 1) * grid_stride;
					let b = (i) * grid_stride;
					let c = grid_stride * grid_stride + i * 4;
					let d = grid_stride * grid_stride + (i - 1) * 4;
					indices.push(a);
					indices.push(b);
					indices.push(c);
					indices.push(a);
					indices.push(c);
					indices.push(d);
				}
				{
					let a = (i - 1) * grid_stride + (grid_stride - 1);
					let b = (i) * grid_stride + (grid_stride - 1);
					let c = grid_stride * grid_stride + i * 4 + 1;
					let d = grid_stride * grid_stride + (i - 1) * 4 + 1;
					indices.push(a);
					indices.push(b);
					indices.push(c);
					indices.push(a);
					indices.push(c);
					indices.push(d);
				}
				{
					let a = i - 1;
					let b = i;
					let c = grid_stride * grid_stride + i * 4 + 2;
					let d = grid_stride * grid_stride + (i - 1) * 4 + 2;
					indices.push(a);
					indices.push(b);
					indices.push(c);
					indices.push(a);
					indices.push(c);
					indices.push(d);
				}
				{
					let a = grid_stride * (grid_stride - 1) + i - 1;
					let b = grid_stride * (grid_stride - 1) + i;
					let c = grid_stride * grid_stride + i * 4 + 3;
					let d = grid_stride * grid_stride + (i - 1) * 4 + 3;
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
			PrimitiveType::TrianglesList,
            &indices
		).unwrap();

		TerrainRenderer {
			shader: shader,
			vertex_buffer: vertex_buffer,
			index_buffer: index_buffer,
		}
	}

	pub fn draw_terrain<Target: Surface>(&self, target: &mut Target, params: &RenderParams, terrain: &Terrain) {

		let nodes = RenderNode::build_tree(terrain, &params.camera);

		for ref node in nodes {
			self.draw_terrain_node(target, params, terrain, node);
		}
	}

	fn draw_terrain_node<Target: Surface>(&self, target: &mut Target, params: &RenderParams, terrain: &Terrain, node: &RenderNode) {
		let transform = params.camera.view_projection_matrix;

		let map = terrain.map.asset.borrow();
		let material = terrain.materials[0].asset.borrow();
		let albedo = material.albedo_map.asset.borrow();

		let inv_lod = 1.0 / node.lod as Real;

		let uniforms = uniform! {
			u_transform: matrix4_to_array(transform),
			u_scale: [terrain.scale.x, terrain.scale.y, terrain.scale.z],
			u_map: map.deref(),
			u_albedo_map: albedo.deref(),
			u_lod_offset: [node.offset.0 as Real * inv_lod, node.offset.1 as Real * inv_lod],
			u_lod_scale: 1.0 * inv_lod,
			u_lines_highlight: 0.0 as Real,
		};

		let mut draw_parameters = params.draw_parameters.clone();

		draw_parameters.depth = Depth {
        	test: DepthTest::IfLess,
        	write: true,
        	.. Default::default()
    	};

		target.draw(&self.vertex_buffer, &self.index_buffer, &self.shader, &uniforms, &draw_parameters).unwrap();
/*
		draw_parameters.depth = Default::default();

   		draw_parameters.polygon_mode = PolygonMode::Line;

		let uniforms = uniform! {
			u_transform: matrix4_to_array(transform),
			u_scale: [terrain.scale.x, terrain.scale.y, terrain.scale.z],
			u_map: map.deref(),
			u_albedo_map: albedo.deref(),
			u_lod_offset: [node.offset.0 as Real * inv_lod, node.offset.1 as Real * inv_lod],
			u_lod_scale: 1.0 * inv_lod,
			u_lines_highlight: 1.0 as Real,
		};

		target.draw(&self.vertex_buffer, &self.index_buffer, &self.shader, &uniforms, &draw_parameters).unwrap();*/
	}


}
