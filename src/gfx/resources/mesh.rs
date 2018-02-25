use glium::{VertexBuffer, IndexBuffer, Display};
use glium::index::PrimitiveType;

use ::assets::AssetRef;
use super::Material;

#[derive(Copy, Clone)]
pub struct MeshVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

implement_vertex!(MeshVertex, position, normal, uv);

pub struct Mesh {
	vertex_buffer: VertexBuffer<MeshVertex>,
	index_buffer: IndexBuffer<u32>,
	pub material: AssetRef<Material>,
}

impl Mesh {

	pub fn new(display: &Display, verticies: &[MeshVertex], indicies: &[u32], material: AssetRef<Material>) -> Self {
		let vertex_buffer = VertexBuffer::new(display, verticies).unwrap();
		let index_buffer = IndexBuffer::new(
			display,
			PrimitiveType::TrianglesList,
            indicies
        ).unwrap();

        Mesh {
        	vertex_buffer: vertex_buffer,
        	index_buffer: index_buffer,
        	material: material,
        }
	}

	pub fn get_buffers(&self) -> (&VertexBuffer<MeshVertex>, &IndexBuffer<u32>) {
		(&self.vertex_buffer, &self.index_buffer)
	}

}
