use glium::{VertexBuffer, IndexBuffer, Display};
use glium::index::PrimitiveType;

#[derive(Copy, Clone)]
pub struct MeshVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

implement_vertex!(MeshVertex, position, normal);

pub struct Mesh {
	vertex_buffer: VertexBuffer<MeshVertex>,
	index_buffer: IndexBuffer<u16>,
}

impl Mesh {

	pub fn new(display: &Display, verticies: &[MeshVertex], indicies: &[u16]) -> Self {
		let vertex_buffer = VertexBuffer::new(display, verticies).unwrap();
		let index_buffer = IndexBuffer::new(
			display,
			PrimitiveType::TrianglesList,
            indicies
        ).unwrap();

        Mesh {
        	vertex_buffer: vertex_buffer,
        	index_buffer: index_buffer,
        }

	}

	pub fn get_buffers(&self) -> (&VertexBuffer<MeshVertex>, &IndexBuffer<u16>) {
		(&self.vertex_buffer, &self.index_buffer)
	}

}
