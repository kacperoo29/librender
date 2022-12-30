use dyn_clone::DynClone;

use crate::platform::opengl::texture::{OglTexture, TextureFormat};

use super::buffer::BufferDataType;

pub trait Texture: Send + Sync + DynClone {
    fn bind(&self, slot: u32);
    fn unbind(&self);
}

dyn_clone::clone_trait_object!(Texture);

pub fn create_texture(
    data: *const u8,
    width: u32,
    height: u32,
    format: TextureFormat,
    data_type: BufferDataType,
) -> Box<dyn Texture> {
    return Box::new(OglTexture::new(width, height, data, format, data_type));
}
