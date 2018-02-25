use glium::{Program, Display, Surface, DrawParameters, VertexBuffer, IndexBuffer, Depth};
use glium::index::PrimitiveType;
use glium::draw_parameters::DepthTest;

use std::ops::Deref;

use ::gfx::scene::CameraRenderingParameters;
use ::terrain::Terrain;
use ::assets::AssetRef;
use ::math::*;

#[derive(Copy, Clone)]
struct TerrainVertex {
	position: [f32; 2],
}

implement_vertex!(TerrainVertex, position);

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

			uniform mat4 transform;
			uniform vec3 scale;
			uniform sampler2D map;

			out vec2 v_uv;

			void main() {
				vec4 map = texture(map, position);
				vec3 terrain_position = vec3(position.x * scale.x, map.r * scale.y, position.y * scale.z);
				gl_Position = transform * vec4(terrain_position, 1.0);
				v_uv = terrain_position.xz;
			}
		"#;

		let fragment_shader_src = r#"
			#version 140

			in vec2 v_uv;

			uniform sampler2D albedo;

			out vec4 o_albedo;
			out vec4 o_normal;

			void main() {
				o_albedo = texture(albedo, v_uv);
				o_normal = vec4(0.0, 1.0, 0.0, 1.0);
			}
		"#;

		let shader = Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

		let mut vertices: Vec<TerrainVertex> = Vec::new();
		let mut indices: Vec<u16> = Vec::new();

		let grid_steps: u16 = 64;
		for x in 0..grid_steps {
			for y in 0..grid_steps {
				let sx = (x as Real) / ((grid_steps - 1) as Real);
				let sy = (y as Real) / ((grid_steps - 1) as Real);
				vertices.push(TerrainVertex {
					position: [sx, sy],
				});
				if x > 0 && y > 0 {
					let a = (x - 1) * grid_steps + (y - 1);
					let b = (x - 1) * grid_steps + (y);
					let c = (x) * grid_steps + (y);
					let d = (x) * grid_steps + (y - 1);
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

	pub fn draw<F: Surface>(&self, target: &mut F, draw_parameters: &DrawParameters, camera: &CameraRenderingParameters, object: &AssetRef<Terrain>) {
		let terrain = object.asset.borrow();

		let transform = camera.view_projection;

		let map = terrain.map.asset.borrow();
		let material = terrain.materials[0].asset.borrow();
		let albedo = material.albedo.asset.borrow();

		let uniforms = uniform! {
			transform: matrix4_to_array(transform),
			map: map.deref(),
			albedo: albedo.deref(),
		};

		let mut draw_parameters = draw_parameters.clone();

		draw_parameters.depth = Depth {
        	test: DepthTest::IfLess,
        	write: true,
        	.. Default::default()
    	};

		target.draw(&self.vertex_buffer, &self.index_buffer, &self.shader, &uniforms, &draw_parameters).unwrap();
	}

}
