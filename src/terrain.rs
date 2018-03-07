use glium::Texture2d;

use ::assets::Asset;
use ::gfx::resources::Material;
use ::math::*;

pub struct Terrain {
	pub map: Asset<Texture2d>,
	pub materials: Vec<Asset<Material>>, 
	pub scale: Vector3,
}

impl Terrain {

	pub fn new(map: Asset<Texture2d>) -> Terrain {
		Terrain {
			map: map,
			materials: Vec::new(),
			scale: vec3(100.0, 20.0, 100.0),
		}
	}

}


