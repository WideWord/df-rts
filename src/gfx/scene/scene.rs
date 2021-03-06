use std::rc::Rc;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};


use ::gfx::scene::{Camera, MeshInstance, Sun};
use ::terrain::Terrain;
use ::assets::Asset;
use ::math::*;

#[derive(Clone)]
pub struct MeshInstanceHandle(pub Rc<MeshInstance>);

impl PartialEq for MeshInstanceHandle {
	fn eq(&self, other: &MeshInstanceHandle) -> bool {
		Rc::ptr_eq(&self.0, &other.0)
	}
}

impl Eq for MeshInstanceHandle {}

impl Hash for MeshInstanceHandle {
	fn hash<H>(&self, state: &mut H) where H: Hasher {
		let ptr = Rc::into_raw(self.0.clone());
    	ptr.hash(state);
		let _ = unsafe{ Rc::from_raw(ptr) };
	}
}

pub struct Scene {
	camera: Camera,
	mesh_instances: HashSet<MeshInstanceHandle>,
	pub terrain: Option<Asset<Terrain>>,
	pub sun: Option<Sun>,
	pub ambient_light: Vector3, 
}

impl Scene {

	pub fn new() -> Self {
		Scene {
			camera: Camera::default(),
			mesh_instances: HashSet::new(),
			terrain: None,
			sun: None,
			ambient_light: vec3(0.1, 0.1, 0.1),
		}
	}

	pub fn add_mesh_instance(&mut self, instance: MeshInstance) -> MeshInstanceHandle {
		let handle = MeshInstanceHandle(Rc::new(instance));
		self.mesh_instances.insert(handle.clone());

		handle
	}

	pub fn get_mesh_instances(&self) -> &HashSet<MeshInstanceHandle> {
		return &self.mesh_instances;
	}

	pub fn camera(&self) -> &Camera {
		return &self.camera
	}

	pub fn camera_mut(&mut self) -> &mut Camera {
		return &mut self.camera
	}

}