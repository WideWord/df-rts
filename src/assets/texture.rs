use image::open as open_image;
use glium::texture::{Texture2d, RawImage2d};

use std::path::{Path, PathBuf};

use ::assets::{Asset, AssetsManager};

pub struct TextureAsset {
	texture: Texture2d,
	path: PathBuf,
}

impl Asset for TextureAsset {

	fn load(assets_manager: &AssetsManager, path: &Path) -> Self {
		let image = open_image(path).unwrap().to_rgba();
		let dimensions = image.dimensions();
		
		let glium_image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimensions);

		let display = assets_manager.get_renderer().get_display();

		let texture = Texture2d::new(display, glium_image).unwrap();

		TextureAsset { 
			texture: texture,
			path: PathBuf::from(path),
		}
	}

	fn get_path(&self) -> &Path {
		self.path.as_path()
	}
}

impl TextureAsset {

	fn get_texture(&self) -> &Texture2d {
		&self.texture
	}

}