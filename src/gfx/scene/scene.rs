use std::rc::Rc;

use std::collections::HashSet;

use super::{Camera, Entity, EntityHandle};


pub struct Scene {
	camera: Camera,
	entities: HashSet<EntityHandle>,
}

impl Scene {

	pub fn new() -> Self {
		Scene {
			camera: Camera::new(),
			entities: HashSet::new(),
		}
	}

	pub fn add_entity(&mut self, entity: Entity) -> EntityHandle {
		let handle = EntityHandle(Rc::new(entity));
		self.entities.insert(handle.clone());

		handle
	}

	pub fn remove_entity(&mut self, entity_handle: EntityHandle) {
		self.entities.remove(&entity_handle);
	}

	pub fn get_entities(&self) -> &HashSet<EntityHandle> {
		return &self.entities;
	}

	pub fn get_camera(&self) -> &Camera {
		return &self.camera
	}

	pub fn set_camera(&mut self, camera: &Camera) {
		self.camera = *camera;
	}

}