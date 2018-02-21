use glium::{Texture2d, Display};

use std::path::{Path, PathBuf};
use std::cell::{RefCell};
use std::rc::Rc;

#[derive(Clone)]
pub struct AssetRef<T> {
	pub asset: Rc<RefCell<T>>,
	pub path: Option<PathBuf>,
}

impl<T> AssetRef<T> {

	pub fn from(asset: T) -> AssetRef<T> {
		AssetRef {
			asset: Rc::new(RefCell::new(asset)),
			path: None,
		}
	}

}

pub fn load_texture(display: &Display, path: &Path) -> AssetRef<Texture2d> {
	use image::open;
	use glium::texture::RawImage2d;

	let image = open(path).unwrap().to_rgba();
	let dimensions = image.dimensions();
		
	let glium_image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimensions);

	let texture = Texture2d::new(display, glium_image).unwrap();

	AssetRef::from(texture)
}
