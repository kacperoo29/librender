use std::{mem::size_of, rc::Rc};

use super::{
    buffer::{create_buffer, create_vertex_array, BufferType, BufferUsage, VertexArray},
    glyph::Glyph,
    mesh::Drawable,
    render_api::RenderAPI,
    shader::{create_shader_from_file, Shader},
};

pub struct DrawableText {
    string: String,
    glyphs: Vec<Glyph>,
    position: (f32, f32),
    scale: f32,

    vertex_array: Box<dyn VertexArray>,
    shader: Box<dyn Shader>,
}

impl Drawable for DrawableText {
    fn draw(&mut self) {
        self.shader.bind();
        self.vertex_array.bind();

        let mut advance = 0.0;
        for glyph in &self.glyphs {
            glyph.texture.bind(0);

            let vertices = glyph.get_vertices(self.position, self.scale, &mut advance);
            self.vertex_array.add_buffer_sub_data(vertices, 0, 0);
            self.vertex_array.draw();
        }
    }

    fn get_shader(&self) -> &Box<dyn Shader> {
        return &self.shader;
    }
}

impl DrawableText {
    pub fn new(
        text: &str,
        pos: (f32, f32),
        scale: f32,
        font_path: &str,
        shader_path: &str,
        render_api: &mut Rc<dyn RenderAPI>,
    ) -> Self {
        let mut vertex_array = create_vertex_array();
        let mut buffer_layout = Vec::new();
        let mut glyphs = Vec::new();

        let glyph_buffer_layout = Glyph::get_buffer_layout();
        buffer_layout.extend(glyph_buffer_layout);

        render_api.disable_align_restrictions();
        for character in text.chars() {
            let glyph = Glyph::new(character, font_path, 48);
            glyphs.push(glyph);
        }

        let vertices: Vec<f32> = Vec::new();
        let vertex_buffer = create_buffer(
            &mut vertices.clone(),
            BufferType::Vertex,
            buffer_layout,
            BufferUsage::Dynamic,
            Some(6 * 4 * size_of::<f32>()),
        );
        render_api.enable_align_restrictions();

        vertex_array.add_buffer(vertex_buffer, BufferType::Vertex);

        let shader = create_shader_from_file(shader_path);

        return Self {
            string: String::from(text),
            glyphs: glyphs,
            position: pos,
            scale: scale,

            vertex_array: vertex_array,
            shader: shader,
        };
    }

    pub fn set_text(&mut self, text: &str, font_path: &str, render_api: &mut Rc<dyn RenderAPI>) {
        self.glyphs.clear();
        if text == self.string {
            return;
        }

        render_api.disable_align_restrictions();
        for character in text.chars() {
            let glyph = Glyph::new(character, font_path, 48);
            self.glyphs.push(glyph);
        }

        render_api.enable_align_restrictions();
    }
}
