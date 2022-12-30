use crate::render::{buffer::BufferDataType, texture::Texture};

use super::vertex_array::OglVertexArray;

#[allow(dead_code)]
pub enum TextureFormat {
    RGBA,
    RGB,
    Red,
    Green,
    Blue,
    Alpha,
}

#[derive(Clone)]
pub struct OglTexture {
    id: u32,
}

impl OglTexture {
    pub fn new(
        width: u32,
        height: u32,
        data: *const u8,
        format: TextureFormat,
        data_type: BufferDataType,
    ) -> OglTexture {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }

        let texture = OglTexture { id };

        let format = Self::translate_format(format);
        let data_type = OglVertexArray::translate_type(data_type);
        texture.bind(0);
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as i32,
                width as i32,
                height as i32,
                0,
                format,
                data_type,
                data as *const std::ffi::c_void,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        }

        return texture;
    }

    fn translate_format(format: TextureFormat) -> u32 {
        match format {
            TextureFormat::RGBA => gl::RGBA,
            TextureFormat::RGB => gl::RGB,
            TextureFormat::Red => gl::RED,
            TextureFormat::Green => gl::GREEN,
            TextureFormat::Blue => gl::BLUE,
            TextureFormat::Alpha => gl::ALPHA,
        }
    }
}

impl Texture for OglTexture {
    fn bind(&self, slot: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}
