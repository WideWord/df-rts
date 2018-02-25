use glium::{Texture2d, Display};

use std::path::{Path, PathBuf};
use std::cell::{RefCell};
use std::rc::Rc;
use std::vec::Vec;

use ::gfx::resources::{Mesh, MeshVertex, Material};

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

pub fn load_mesh(display: &Display, path: &Path, material: AssetRef<Material>) -> AssetRef<Mesh> {
	use assimp::import::Importer;

	let importer = Importer::new();

	let path_str = path.to_string_lossy().into_owned();
	let scene = importer.read_file(&path_str).unwrap();

	let ai_mesh = scene.mesh(0).unwrap();

	let mut vertices: Vec<MeshVertex> = Vec::new();

	for i in 0..ai_mesh.num_vertices() {
		let position = ai_mesh.get_vertex(i).unwrap();
		let normal = ai_mesh.get_normal(i).unwrap();
		let uv = ai_mesh.get_texture_coord(0, i).unwrap();

		vertices.push(MeshVertex {
			position: [position.x, position.y, position.z],
			normal: [normal.x, normal.y, normal.z],
			uv: [uv.x, uv.y],
		});
	}

	let mut indicies: Vec<u32> = Vec::new();

	for i in 0..ai_mesh.num_faces() {
		let face = ai_mesh.get_face(i).unwrap();
		indicies.push(face[0]);
		indicies.push(face[1]);
		indicies.push(face[2]);
	}

	AssetRef::from(Mesh::new(display, &vertices, &indicies, material))
}
