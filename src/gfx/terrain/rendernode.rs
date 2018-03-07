use std::collections::VecDeque;

use ::math::*;
use ::terrain::Terrain;
use ::gfx::scene::{CameraRenderParams};

pub struct RenderNode {
	pub offset: (u16, u16),
	pub lod: u16,
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
		});

		while let Some(node) = queue.pop_front() {
			
			if intersect_frustum_aabb(&camera.frustum, &node.bounds(terrain)) != IntersectionTestResult::Outside {
				
				let node_size = terrain.scale.x / (node.lod as Real);

				if node_size < 50.0 || terrain.scale.x / (node.lod as Real) * 1.3 < camera.spatial.position.distance(node.bounds(terrain).center()) {
					result.push(node);
				} else {
					let lod = node.lod * 2;
					let offset = (node.offset.0 * 2, node.offset.1 * 2);

					queue.push_back(RenderNode {
						lod: lod,
						offset: offset,
					});

					queue.push_back(RenderNode {
						lod: lod,
						offset: (offset.0 + 1, offset.1),
					});

					queue.push_back(RenderNode {
						lod: lod,
						offset: (offset.0 + 1, offset.1 + 1),
					});

					queue.push_back(RenderNode {
						lod: lod,
						offset: (offset.0, offset.1 + 1),
					});
				}
			}

		}

		result
	}

}





