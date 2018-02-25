use ::math::Spatial;
use ::gfx::resources::Mesh;
use ::assets::Asset;

pub struct MeshInstance {
	pub spatial: Spatial,
	pub is_static: bool,
	pub mesh: Asset<Mesh>,
}

