use std::cell::RefCell;
use std::rc::Rc;

use ::math::Spatial;
use ::gfx::Mesh;

pub struct MeshInstance {
	pub spatial: Spatial,
	pub is_static: bool,
	pub mesh: Rc<RefCell<Mesh>>,
}

