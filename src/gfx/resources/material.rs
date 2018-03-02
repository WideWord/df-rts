use glium::Texture2d;

use ::assets::Asset;
use ::math::*;

pub struct Material {
	pub albedo_map: Asset<Texture2d>,
	pub roughness: Real,
	pub metallic: Real,
}
