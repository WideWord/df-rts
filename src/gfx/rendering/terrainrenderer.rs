use glium::{Program, Display, Surface, VertexBuffer, IndexBuffer, Depth};
use glium::index::PrimitiveType;
use glium::draw_parameters::{DepthTest, PolygonMode};

use std::ops::Deref;

use ::gfx::rendering::RenderParameters;
use ::terrain::Terrain;
use ::math::*;

#[derive(Copy, Clone)]
struct TerrainVertex {
	position: [f32; 2],
}

implement_vertex!(TerrainVertex, position);

pub struct TerrainRenderer {
	shader: Program,
	vertex_buffer: VertexBuffer<TerrainVertex>,
	index_buffers: Vec<IndexBuffer<u16>>,
}

impl TerrainRenderer {

	pub fn new(display: &Display) -> Self {
		let vertex_shader_src = r#"
			#version 140

			in vec2 position;

			uniform mat4 u_transform;
			uniform vec3 u_scale;
			uniform vec2 u_lod_offset;
			uniform float u_lod_scale;
			uniform vec3 u_world_offset;
			uniform sampler2D u_map;

			out vec2 v_uv;
			out vec3 v_normal;


			void main() {
				vec2 lod_position = position * u_lod_scale + u_lod_offset;
				vec4 map = texture(u_map, lod_position);
				vec3 terrain_position = vec3(lod_position.x * u_scale.x, map.r * u_scale.y, lod_position.y * u_scale.z);
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

		let mut vertices: Vec<TerrainVertex> = Vec::new();

		let grid_steps: u16 = 8;
		for x in 0..grid_steps {
			for y in 0..grid_steps {
				let sx = (x as Real) / ((grid_steps - 1) as Real);
				let sy = (y as Real) / ((grid_steps - 1) as Real);
				vertices.push(TerrainVertex {
					position: [sx, sy],
				});
			}
		}

		let mut common_indices: Vec<u16> = Vec::new();

		for x in 2..(grid_steps-1) {
			for y in 2..(grid_steps-1) {
				let a = (x - 1) * grid_steps + (y - 1);
				let b = (x - 1) * grid_steps + (y);
				let c = (x) * grid_steps + (y);
				let d = (x) * grid_steps + (y - 1);
				common_indices.push(a);
				common_indices.push(b);
				common_indices.push(c);
				common_indices.push(a);
				common_indices.push(c);
				common_indices.push(d);
			}
		}

		let mut index_buffers: Vec<IndexBuffer<u16>> = Vec::new();

		// setted bit means a seam to higher level lod at the corresponding side
		for seam_config_id in 0..16 {
			let mut indices = common_indices.clone();

			for side in 0..4 { // -x, +x, -z, +z
				if seam_config_id & (1 << side) == 0 {
					for i in 1..grid_steps {
						let (a, b, c, d) = match side {
							0 => (
								0 * grid_steps + (grid_steps - (i - 1) - 1),
								0 * grid_steps + (grid_steps - i - 1),
								1 * grid_steps + (grid_steps - i - 1),
								1 * grid_steps + (grid_steps - (i - 1) - 1),
							),
							1 => (
								(grid_steps - 1) * grid_steps + (i - 1),
								(grid_steps - 1) * grid_steps + i,
								(grid_steps - 2) * grid_steps + i,
								(grid_steps - 2) * grid_steps + (i - 1),
							),
							2 => (
								(i - 1) * grid_steps + 0,
								i * grid_steps + 0,
								i * grid_steps + 1,
								(i - 1) * grid_steps + 1,
							),
							3 => (
								(grid_steps - (i - 1) - 1) * grid_steps + (grid_steps - 1),
								(grid_steps - i - 1) * grid_steps + (grid_steps - 1),
								(grid_steps - i - 1) * grid_steps + (grid_steps - 2),
								(grid_steps - (i - 1) - 1) * grid_steps + (grid_steps - 2),
							),
							_ => unreachable!(),
						};

						if i == grid_steps - 1 {
							indices.push(a);
							indices.push(b);
							indices.push(d);
						} else {
							indices.push(a);
							indices.push(b);
							indices.push(c);
							if i > 1 {
								indices.push(a);
								indices.push(c);
								indices.push(d);
							}
						}
					}
				}
				
			}

			index_buffers.push(IndexBuffer::new(
				display,
				PrimitiveType::TrianglesList,
	            &indices
			).unwrap());
		}

		let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();

		TerrainRenderer {
			shader: shader,
			vertex_buffer: vertex_buffer,
			index_buffers: index_buffers,
		}
	}

	pub fn draw_terrain<Target: Surface>(&self, target: &mut Target, params: &RenderParameters, terrain: &Terrain) {
		println!("\n\nterrain drawing...");

		self.draw_terrain_subdivision(target, params, terrain, 
			vec2(0.0, 0.0), 1.0, AABB3 { min: vec3(0.0, 0.0, 0.0), max: terrain.scale });
	}

	fn draw_terrain_subdivision<Target: Surface>(&self, target: &mut Target, params: &RenderParameters, terrain: &Terrain, sd_offset: Vector2, sd_scale: Real, sd_bounds: AABB3) {
		print!("terrain subdiv {:?} {:?}", sd_offset, sd_scale);

		let camera = &params.camera;
		if sd_scale * terrain.scale.x < 10.0 {
			self.draw_terrain_lod(target, params, terrain, sd_offset, sd_scale);
			println!(" drawed");
		} else if camera.spatial.position.distance2(sd_bounds.center()) > (sd_scale * terrain.scale.x * 1.5).powi(2) {
			self.draw_terrain_lod(target, params, terrain, sd_offset, sd_scale);
			println!(" drawed");
		} else if intersect_frustum_aabb(&camera.frustum, &sd_bounds) != IntersectionTestResult::Outside {
			println!(" subdivided");
			let new_sd_scale = sd_scale * 0.5;
			let sd_bounds_mid_x = sd_bounds.min.x + (sd_bounds.max.x - sd_bounds.min.x) * 0.5;
			let sd_bounds_mid_z = sd_bounds.min.z + (sd_bounds.max.z - sd_bounds.min.z) * 0.5;
			self.draw_terrain_subdivision(target, params, terrain, 
				sd_offset,
				new_sd_scale, 
				AABB3 { min: sd_bounds.min, max: vec3(sd_bounds_mid_x, sd_bounds.max.y, sd_bounds_mid_z) });

			self.draw_terrain_subdivision(target, params, terrain, 
				sd_offset + vec2(new_sd_scale, 0.0), 
				new_sd_scale, 
				AABB3 { min: vec3(sd_bounds_mid_x, sd_bounds.min.y, sd_bounds.min.z), max: vec3(sd_bounds.max.x, sd_bounds.max.y, sd_bounds_mid_z) });					
			
			self.draw_terrain_subdivision(target, params, terrain, 
				sd_offset + vec2(0.0, new_sd_scale), 
				new_sd_scale, 
				AABB3 { min: vec3(sd_bounds.min.x, sd_bounds.min.y, sd_bounds_mid_z), max: vec3(sd_bounds_mid_x, sd_bounds.max.y, sd_bounds.max.z) });					

			self.draw_terrain_subdivision(target, params, terrain, 
				sd_offset + vec2(new_sd_scale, new_sd_scale), 
				new_sd_scale, 
				AABB3 { min: vec3(sd_bounds_mid_x, sd_bounds.min.y, sd_bounds_mid_z), max: sd_bounds.max });
		}
	}

	fn draw_terrain_lod<Target: Surface>(&self, target: &mut Target, params: &RenderParameters, terrain: &Terrain, lod_offset: Vector2, lod_scale: Real) {
		let transform = params.camera.view_projection_matrix;

		let map = terrain.map.asset.borrow();
		let material = terrain.materials[0].asset.borrow();
		let albedo = material.albedo_map.asset.borrow();

		let uniforms = uniform! {
			u_transform: matrix4_to_array(transform),
			u_scale: [terrain.scale.x, terrain.scale.y, terrain.scale.z],
			u_map: map.deref(),
			u_albedo_map: albedo.deref(),
			u_lod_offset: [lod_offset.x, lod_offset.y],
			u_lod_scale: lod_scale,
			u_lines_highlight: 0.0 as Real,
		};

		let mut draw_parameters = params.draw_parameters.clone();

		draw_parameters.depth = Depth {
        	test: DepthTest::IfLess,
        	write: true,
        	.. Default::default()
    	};

		target.draw(&self.vertex_buffer, &self.index_buffers[0], &self.shader, &uniforms, &draw_parameters).unwrap();

		draw_parameters.depth = Depth {
        	test: DepthTest::IfLessOrEqual,
        	write: true,
        	.. Default::default()
    	};

   		draw_parameters.polygon_mode = PolygonMode::Line;

		let uniforms = uniform! {
			u_transform: matrix4_to_array(transform),
			u_scale: [terrain.scale.x, terrain.scale.y, terrain.scale.z],
			u_map: map.deref(),
			u_albedo_map: albedo.deref(),
			u_lod_offset: [lod_offset.x, lod_offset.y],
			u_lod_scale: lod_scale,
			u_lines_highlight: 1.0 as Real,
		};

		target.draw(&self.vertex_buffer, &self.index_buffers[0], &self.shader, &uniforms, &draw_parameters).unwrap();

	}

}
