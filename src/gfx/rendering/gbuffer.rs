use glium::framebuffer::MultiOutputFrameBuffer;
use glium::{Texture2d, Display};
use glium::texture::{UncompressedFloatFormat, MipmapsOption, DepthTexture2d};

pub struct GBuffer {
	albedo_texture: Texture2d,
	normal_texture: Texture2d,
	depth_texture: DepthTexture2d,
}

impl GBuffer {

	pub fn new(display: &Display, size: (u32, u32)) -> Self {
		
		let albedo_texture = Texture2d::empty_with_format(display, UncompressedFloatFormat::U8U8U8U8, MipmapsOption::NoMipmap, size.0, size.1).unwrap();

		let normal_texture = Texture2d::empty_with_format(display, UncompressedFloatFormat::U8U8U8U8, MipmapsOption::NoMipmap, size.0, size.1).unwrap();

		let depth_texture = DepthTexture2d::empty(display, size.0, size.1).unwrap();

		GBuffer {
			albedo_texture: albedo_texture,
			normal_texture: normal_texture,
			depth_texture: depth_texture,
		}
	}

	pub fn framebuffer(&self, display: &Display) -> MultiOutputFrameBuffer {
		let attachments = [
			("o_albedo_metallic", &self.albedo_texture),
			("o_normal_roughness", &self.normal_texture),
		];

		let framebuffer = MultiOutputFrameBuffer::with_depth_buffer(display, attachments.iter().cloned(), &self.depth_texture).unwrap();

		framebuffer
	}

	pub fn albedo_metallic_texture(&self) -> &Texture2d {
		return &self.albedo_texture;
	}

	pub fn normal_roughness_texture(&self) -> &Texture2d {
		return &self.normal_texture;
	}

	pub fn depth_texture(&self) -> &DepthTexture2d {
		return &self.depth_texture;
	}

}
