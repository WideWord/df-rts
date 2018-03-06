use enum_map::EnumMap;
use std::collections::VecDeque;

use ::gfx::rendering::RenderParams;
use ::terrain::Terrain;
use ::math::*;

pub struct TerrainRenderTree {
	root: TerrainRenderNode,
}

impl TerrainRenderTree {

	pub fn build(render_params: &RenderParams, terrain: &Terrain) -> Self {

		let mut root = TerrainRenderNode {
			offset: vec2(0.0, 0.0),
			scale: 1.0,
			bounds: AABB3 { min: vec3(0.0, 0.0, 0.0), max: terrain.scale },
			to_draw: false,
			seam_config: 0,
			level_of_detail: 0,
			children: None,
		};

		root.process_node(render_params, terrain);

		TerrainRenderTree {
			root: root,
		}
	}

	pub fn draw_iter<'a>(&'a self) -> TerrainRenderTreeDrawIterator<'a> {
		let mut queue: VecDeque<&'a TerrainRenderNode> = VecDeque::new();
		queue.push_front(&self.root);

		TerrainRenderTreeDrawIterator {
			queue: queue,
		}
	}

}

pub struct TerrainRenderTreeDrawIterator<'a> {
	queue: VecDeque<&'a TerrainRenderNode>,
}

impl<'a> Iterator for TerrainRenderTreeDrawIterator<'a> {

	type Item = &'a TerrainRenderNode;
	
	fn next(&mut self) -> Option<&'a TerrainRenderNode> {
		loop {
			if let Some(ref node) = self.queue.pop_front() {
				if node.to_draw {
					return Some(node);
				} else if let Some(ref children) = node.children {
					for (_, ref child) in children.iter() {
						self.queue.push_front(child);
					}
				}
			} else {
				return None;
			}
		}
	}
}





pub struct TerrainRenderNode {
	pub offset: Vector2,
	pub scale: Real,
	bounds: AABB3,
	to_draw: bool,
	pub seam_config: u8,
	level_of_detail: u8,
	children: Option<Box<EnumMap<TerrainRenderNodeChildPosition, TerrainRenderNode>>>,
}

#[derive(EnumMap)]
enum TerrainRenderNodeChildPosition {
	TopLeft,
	TopRight,
	BottomLeft,
	BottomRight,
}

enum TerrainRenderNodeLookupDirection {
	Top, Left, Bottom, Right
}

impl TerrainRenderNode {

	fn process_node(&mut self, render_params: &RenderParams, terrain: &Terrain) {
		let camera = &render_params.camera;
		if self.scale * terrain.scale.x < 10.0 {
			self.to_draw = true;
		} else if camera.spatial.position.distance2(self.bounds.center()) > (self.scale * terrain.scale.x * 1.2).powi(2) {
			self.to_draw = true;
		} else if intersect_frustum_aabb(&camera.frustum, &self.bounds) != IntersectionTestResult::Outside {
			let new_scale = self.scale * 0.5;

			let min = self.bounds.min;
			let max = self.bounds.max;
			let mid = self.bounds.center();

			let mut children = enum_map! {
				TerrainRenderNodeChildPosition::BottomLeft => TerrainRenderNode {
					offset: self.offset,
					scale: new_scale,
					bounds: AABB3 { min: vec3(min.x, min.y, min.z), max: vec3(mid.x, max.y, mid.z) },
					to_draw: false,
					seam_config: 0,
					level_of_detail: self.level_of_detail + 1,
					children: None,
				},
				TerrainRenderNodeChildPosition::BottomRight => TerrainRenderNode {
					offset: self.offset + vec2(new_scale, 0.0),
					scale: new_scale,
					bounds: AABB3 { min: vec3(mid.x, min.y, min.z), max: vec3(max.x, max.y, mid.z) },
					to_draw: false,
					seam_config: 0,
					level_of_detail: self.level_of_detail + 1,
					children: None,
				},
				TerrainRenderNodeChildPosition::TopRight => TerrainRenderNode {
					offset: self.offset + vec2(new_scale, new_scale),
					scale: new_scale,
					bounds: AABB3 { min: vec3(mid.x, min.y, mid.z), max: vec3(max.x, max.y, max.z) },
					to_draw: false,
					seam_config: 0,
					level_of_detail: self.level_of_detail + 1,
					children: None,
				},
				TerrainRenderNodeChildPosition::TopLeft => TerrainRenderNode {
					offset: self.offset + vec2(0.0, new_scale),
					scale: new_scale,
					bounds: AABB3 { min: vec3(min.x, min.y, mid.z), max: vec3(mid.x, max.y, max.z) },
					to_draw: false,
					seam_config: 0,
					level_of_detail: self.level_of_detail + 1,
					children: None,
				},
			};

			for (_, child) in &mut children {
				child.process_node(render_params, terrain);
			}

			self.children = Some(Box::new(children));
		}
	}

}
