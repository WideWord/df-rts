use glium::Texture2d;

use ::assets::AssetRef;

pub struct Material {
	pub albedo: AssetRef<Texture2d>,
}
