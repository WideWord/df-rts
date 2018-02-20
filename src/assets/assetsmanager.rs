
use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::borrow::Borrow;

use ::gfx::Renderer;
use ::assets::{Asset, TextureAsset};

struct AssetsContainer<T: Asset> {
	assets: RefCell<HashMap<PathBuf, Weak<RefCell<T>>>>,
}

impl<T: Asset> AssetsContainer<T> {

	fn new() -> Self {
		AssetsContainer {
			assets: RefCell::new(HashMap::new()),
		}
	}

	fn get(&self, assets_manager: &AssetsManager, path: &Path) -> Rc<RefCell<T>> {
		let maybe_asset = self.assets.borrow().get(path).map(|a| { a.clone() });
		match maybe_asset {
			Some(weak_asset) => match weak_asset.upgrade() {
				Some(asset) => asset.clone(),
				None => self.load(assets_manager, path),
			},
			None => self.load(assets_manager, path)
		}
	}

	fn load(&self, assets_manager: &AssetsManager, path: &Path) -> Rc<RefCell<T>> {
		let result = Rc::new(RefCell::new(T::load(assets_manager, path)));
		self.assets.borrow_mut().insert(PathBuf::from(path), Rc::downgrade(&result));

		result
	}


}

pub struct AssetsManager {
	textures: AssetsContainer<TextureAsset>,
	renderer: Rc<Renderer>,
}

impl AssetsManager {

	pub fn new(renderer: Rc<Renderer>) -> Self {
		AssetsManager {
			textures: AssetsContainer::new(),
			renderer: renderer,
		}
	}

	pub fn get_renderer(&self) -> &Renderer {
		self.renderer.borrow()
	}

	pub fn get_texture(&self, path: &Path) -> Rc<RefCell<TextureAsset>> {
		self.textures.get(&self, path)
	}


}