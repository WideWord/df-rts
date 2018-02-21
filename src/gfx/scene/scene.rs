use super::Camera;

pub struct Scene {
	camera: Camera,
}

impl Scene {

	pub fn new() -> Self {
		Scene {
			camera: Camera::new(),
		}
	}

}