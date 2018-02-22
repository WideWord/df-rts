use glium::framebuffer::MultiOutputFrameBuffer;
use glium::{Texture2d, Display};
use glium::texture::{UncompressedFloatFormat, MipmapsOption};

pub struct GBuffer {
	albedo_texture: Texture2d,
}

impl GBuffer {

	pub fn new(display: &Display, size: (u32, u32)) -> Self {
		
		let albedo_texture = Texture2d::empty_with_format(display, UncompressedFloatFormat::U8U8U8, MipmapsOption::NoMipmap, size.0, size.1).unwrap();

		GBuffer {
			albedo_texture: albedo_texture,
		}
	}

	pub fn get_framebuffer(&self, display: &Display) -> MultiOutputFrameBuffer {
		let attachments = [("o_albedo", &self.albedo_texture)];

		let framebuffer = MultiOutputFrameBuffer::new(display, attachments.iter().cloned()).unwrap();

		framebuffer
	}

	pub fn get_albedo_texture(&self) -> &Texture2d {
		return &self.albedo_texture;
	}

}
