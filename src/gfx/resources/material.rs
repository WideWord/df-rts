use glium::Texture2d;

use ::assets::Asset;

pub struct Material {
	pub albedo_map: Asset<Texture2d>,
	pub roughness_map: Asset<Texture2d>,
	pub metallic_map: Asset<Texture2d>,
}
