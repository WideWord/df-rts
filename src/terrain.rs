use glium::Texture2d;

use ::assets::AssetRef;

pub struct Terrain {
	pub map: AssetRef<Texture2d>,
}

impl Terrain {

	pub fn new(map: AssetRef<Texture2d>) -> Terrain {
		Terrain {
			map: map
		}
	}

}


