use crate::platform::opengl::{buffer::OglBuffer, vertex_array::OglVertexArray};

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
    Vertex,
    Index,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum BufferDataType {
    F32,
    F64,
    I8,
    I16,
    I32,
    U8,
    U16,
    U32
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum BufferUsage {
    Static,
    Dynamic,
    Stream
}

#[derive(Clone, Copy)]
pub struct BufferLayoutNode {
    pub elements: u32,
    pub data_type: BufferDataType,
    pub normalized: bool,
}

impl BufferLayoutNode {
    pub fn get_layout_size(layout: Vec<BufferLayoutNode>) -> usize {
        let mut size = 0;
        for node in layout {
            size += node.elements as usize;
        }
        return size;
    }
}

pub trait Bindable {
    fn bind(&self);
    fn unbind(&self);
}

pub trait Buffer: Bindable {
    fn get_layout(&self) -> &Vec<BufferLayoutNode>;
    fn get_element_count(&self) -> usize;
    // TODO: Make generic
    fn add_sub_data(&mut self, data: Vec<f32>, offset: usize);
}

pub trait VertexArray: Bindable {
    fn draw(&self);
    fn add_buffer(&mut self, buffer: Box<dyn Buffer>, buffer_type: BufferType);
    fn get_buffers(&self) -> &Vec<Box<dyn Buffer>>;
    // TODO: Make generic
    fn add_buffer_sub_data(&mut self, data: Vec<f32>, offset: usize, buffer_index: usize);
}

pub fn create_buffer<T>(
    data: &mut Vec<T>,
    buffer_type: BufferType,
    buffer_layout: Vec<BufferLayoutNode>,
    buffer_usage: BufferUsage,
    size: Option<usize>
) -> Box<dyn Buffer> {
    return Box::new(OglBuffer::new(data, buffer_type, buffer_layout, buffer_usage, size));
}

pub fn create_vertex_array() -> Box<dyn VertexArray> {
    return Box::new(OglVertexArray::new());
}
