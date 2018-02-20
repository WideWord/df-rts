use std::path::Path;

use ::assets::AssetsManager;

pub trait Asset {
	fn load(assets_manager: &AssetsManager, path: &Path) -> Self;
	fn get_path(&self) -> &Path;
}
