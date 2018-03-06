use std::collections::VecDeque;

use ::math::*;
use ::terrain::Terrain;
use ::gfx::scene::{CameraRenderParams};

pub struct RenderNode {
	pub offset: (u16, u16),
	pub lod: u16,
	pub seam: u8,
}

impl RenderNode {

	pub fn bounds(&self, terrain: &Terrain) -> AABB3 {
		let step = terrain.scale / (self.lod as Real);
		AABB3 { 
			min: vec3(step.x * (self.offset.0 as Real), 		0.0, 				step.z * (self.offset.1 as Real)),
			max: vec3(step.x * ((self.offset.0 + 1) as Real), 	terrain.scale.y, 	step.z * ((self.offset.1 + 1) as Real)),
		}
	}


	pub fn build_tree(terrain: &Terrain, camera: &CameraRenderParams) -> Vec<RenderNode> {

		let mut result = Vec::<RenderNode>::new();
		let mut queue = VecDeque::new();

		queue.push_back(RenderNode {
			offset: (0, 0),
			lod: 1,
			seam: 0,
		});

		while let Some(mut node) = queue.pop_front() {
			
			if intersect_frustum_aabb(&camera.frustum, &node.bounds(terrain)) != IntersectionTestResult::Outside {
			
				let is_node_small = terrain.scale.x / (node.lod as Real) * 1.2 < camera.spatial.position.distance(node.bounds(terrain).center());

				let mut is_node_can_be_subdivided = true;
				for j in &result {

					if j.lod * 2 != node.lod { continue }

					let dx = -((j.offset.0 * 2) as i16) + (node.offset.0 as i16);
					let dz = -((j.offset.1 * 2) as i16) + (node.offset.1 as i16);

					if dz == 0 || dz == 1 {
						if dx == -1 {
							node.seam |= 2;
							is_node_can_be_subdivided = false;
						} else if dx == 2 {
							node.seam |= 1;
							is_node_can_be_subdivided = false;
						}
					} else if dx == 0 || dx == 1 {
						if dz == -1 {
							node.seam |= 3;
							is_node_can_be_subdivided = false;
						} else if dz == 2 {
							node.seam |= 4;
							is_node_can_be_subdivided = false;
						}
					}
				}

				if !is_node_can_be_subdivided || is_node_small  {
					result.push(node);		
				} else {
					let lod = node.lod * 2;
					let offset = (node.offset.0 * 2, node.offset.1 * 2);

					queue.push_back(RenderNode {
						lod: lod,
						offset: offset,
						seam: 0,
					});

					queue.push_back(RenderNode {
						lod: lod,
						offset: (offset.0 + 1, offset.1),
						seam: 0,
					});

					queue.push_back(RenderNode {
						lod: lod,
						offset: (offset.0 + 1, offset.1 + 1),
						seam: 0,
					});

					queue.push_back(RenderNode {
						lod: lod,
						offset: (offset.0, offset.1 + 1),
						seam: 0,
					});
				}
			}

		}

		result
	}

}





