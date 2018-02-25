use glium::Texture2d;

use ::assets::Asset;

pub struct Material {
	pub albedo: Asset<Texture2d>,
}
