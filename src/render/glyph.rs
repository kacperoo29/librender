use dashmap::DashMap;
use once_cell::sync::Lazy;

use crate::platform::opengl::texture::TextureFormat;

use super::{
    buffer::{BufferDataType, BufferLayoutNode},
    texture::{create_texture, Texture},
};

// TODO: Different fonts, make glyph factory.
static GLYPH_CACHE: Lazy<DashMap<char, Glyph>> = Lazy::new(|| DashMap::new());

#[derive(Clone)]
pub struct Glyph {
    pub texture: Box<dyn Texture>,
    pub size: (u32, u32),
    pub bearing: (u32, u32),
    pub advance: u32,
}

impl Glyph {
    pub fn new(character: char, font_path: &str, font_size: u32) -> Glyph {
        let cached_glyph = GLYPH_CACHE.get(&character);
        if cached_glyph.is_some() {
            return cached_glyph.unwrap().clone();
        }

        let lib = freetype::Library::init().unwrap();
        let face = lib.new_face(font_path, 0).unwrap();
        face.set_char_size((font_size * 64) as isize, (font_size * 64) as isize, 96, 96)
            .unwrap();
        face.load_char(character as usize, freetype::face::LoadFlag::RENDER)
            .unwrap();

        let glyph = face.glyph();
        let bitmap = glyph.bitmap();

        let texture = create_texture(
            bitmap.buffer().as_ptr(),
            bitmap.width() as u32,
            bitmap.rows() as u32,
            TextureFormat::Red,
            BufferDataType::U8,
        );

        GLYPH_CACHE.insert(
            character,
            Glyph {
                texture,
                size: (bitmap.width() as u32, bitmap.rows() as u32),
                bearing: (glyph.bitmap_left() as u32, glyph.bitmap_top() as u32),
                advance: glyph.advance().x as u32,
            },
        );

        return GLYPH_CACHE.get(&character).unwrap().clone();
    }

    pub fn get_buffer_layout() -> Vec<BufferLayoutNode> {
        return vec![BufferLayoutNode {
            elements: 4,
            data_type: BufferDataType::F32,
            normalized: false,
        }];
    }

    pub fn get_vertices(&self, pos: (f32, f32), scale: f32, advance: &mut f32) -> Vec<f32> {
        let mut vertices = Vec::new();

        let x = pos.0 + *advance + (self.bearing.0 as f32 * scale);
        let y = pos.1 - (self.size.1 as f32 - self.bearing.1 as f32) * scale;

        let width = self.size.0 as f32 * scale;
        let height = self.size.1 as f32 * scale;

        *advance += (self.advance >> 6) as f32 * scale;

        vertices.push(x);
        vertices.push(y + height);
        vertices.push(0.0);
        vertices.push(0.0);

        vertices.push(x);
        vertices.push(y);
        vertices.push(0.0);
        vertices.push(1.0);

        vertices.push(x + width);
        vertices.push(y);
        vertices.push(1.0);
        vertices.push(1.0);

        vertices.push(x);
        vertices.push(y + height);
        vertices.push(0.0);
        vertices.push(0.0);

        vertices.push(x + width);
        vertices.push(y);
        vertices.push(1.0);
        vertices.push(1.0);

        vertices.push(x + width);
        vertices.push(y + height);
        vertices.push(1.0);
        vertices.push(0.0);

        return vertices;
    }
}
