use super::{
    buffer::{
        create_buffer, create_vertex_array, BufferLayoutNode, BufferType, BufferUsage,
        VertexArray,
    },
    shader::{create_shader_from_file, Shader},
    texture::Texture
};

pub trait Drawable {
    fn draw(&mut self);
    fn get_shader(&self) -> &Box<dyn Shader>;
}

pub struct Mesh {
    vertex_array: Box<dyn VertexArray>,    
    shader: Box<dyn Shader>,
    texture: Option<Box<dyn Texture>>,
}

impl Drawable for Mesh {
    fn draw(&mut self) {
        self.shader.bind();
        self.vertex_array.bind();

        if let Some(texture) = &self.texture {
            texture.bind(0);
        }

        self.vertex_array.draw();
    }

    fn get_shader(&self) -> &Box<dyn Shader> {
        return &self.shader;
    }
}

impl Mesh {
    pub fn new(vertices: Vec<f32>, buffer_layout: Vec<BufferLayoutNode>, shader_path: &str) -> Self {
        let mut vertex_array = create_vertex_array();
        let vertex_buffer = create_buffer(
            &mut vertices.clone(),
            BufferType::Vertex,
            buffer_layout,
            BufferUsage::Static,
            None,
        );

        vertex_array.add_buffer(vertex_buffer, BufferType::Vertex);

        let shader = create_shader_from_file(shader_path);

        return Self {
            vertex_array: vertex_array,
            shader: shader,
            texture: None,
        };
    }

    pub fn new_indexed(vertices: Vec<f32>, indices: Vec<u32>, buffer_layout: Vec<BufferLayoutNode>, shader_path: &str) -> Self {
        let mut vertex_array = create_vertex_array();
        let vertex_buffer = create_buffer(
            &mut vertices.clone(),
            BufferType::Vertex,
            buffer_layout,
            BufferUsage::Static,
            None,
        );

        let index_buffer = create_buffer(
            &mut indices.clone(),
            BufferType::Index,
            Vec::new(),
            BufferUsage::Static,
            None,
        );

        vertex_array.add_buffer(vertex_buffer, BufferType::Vertex);
        vertex_array.add_buffer(index_buffer, BufferType::Index);

        let shader = create_shader_from_file(shader_path);

        return Self {
            vertex_array: vertex_array,
            shader: shader,
            texture: None,
        };
    }
}
