use std::cell::RefCell;
use std::rc::Rc;
use std::hash::{Hash, Hasher};

use ::math::Spatial;
use ::gfx::Mesh;

pub struct Entity {
	pub spatial: Spatial,
	pub is_static: bool,
	pub mesh: Rc<RefCell<Mesh>>,
}

#[derive(Clone)]
pub struct EntityHandle(pub Rc<Entity>);

impl PartialEq for EntityHandle {
	fn eq(&self, other: &EntityHandle) -> bool {
		Rc::ptr_eq(&self.0, &other.0)
	}
}

impl Eq for EntityHandle {}

impl Hash for EntityHandle {
	fn hash<H>(&self, state: &mut H) where H: Hasher {
		let ptr = Rc::into_raw(self.0.clone());
    	ptr.hash(state);
		let _ = unsafe{ Rc::from_raw(ptr) };
	}
}
