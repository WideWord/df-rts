use ::math::Spatial;
use ::gfx::resources::Mesh;
use ::assets::AssetRef;

pub struct MeshInstance {
	pub spatial: Spatial,
	pub is_static: bool,
	pub mesh: AssetRef<Mesh>,
}

