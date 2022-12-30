use std::{ffi::c_void, mem::size_of};

use gl::types;

use crate::render::buffer::{Bindable, Buffer, BufferLayoutNode, BufferType, BufferUsage};

pub struct OglBuffer {
    id: u32,
    buffer_type: types::GLenum,
    buffer_layout: Vec<BufferLayoutNode>,
    element_count: usize,
}

impl OglBuffer {
    pub fn new<T>(
        data: &mut Vec<T>,
        buffer_type: BufferType,
        buffer_layout: Vec<BufferLayoutNode>,
        buffer_usage: BufferUsage,
        size: Option<usize>,
    ) -> OglBuffer {
        let mut buffer = OglBuffer {
            id: 0,
            buffer_type: match buffer_type {
                BufferType::Vertex => gl::ARRAY_BUFFER,
                BufferType::Index => gl::ELEMENT_ARRAY_BUFFER,
            },
            element_count: OglBuffer::calculate_element_count(
                data.len(),
                buffer_layout.clone(),
                buffer_type,
            ),
            buffer_layout: buffer_layout,
        };
        unsafe {
            gl::GenBuffers(1, &mut buffer.id);
            gl::BindBuffer(buffer.buffer_type, buffer.id);
            gl::BufferData(
                buffer.buffer_type,
                if size.is_some() {
                    size.unwrap() as isize
                } else {
                    (data.len() * size_of::<T>()) as isize
                },
                if (data.len() * size_of::<T>()) > 0 {
                    data.as_ptr() as *const c_void
                } else {
                    std::ptr::null()
                },
                Self::translate_usage(buffer_usage),
            );
        }

        return buffer;
    }

    fn calculate_element_count(
        data_len: usize,
        layout: Vec<BufferLayoutNode>,
        buffer_type: BufferType,
    ) -> usize {
        let layout_size = BufferLayoutNode::get_layout_size(layout);
        if buffer_type == BufferType::Vertex {
            return data_len / layout_size;
        } else {
            return data_len;
        }
    }

    fn translate_usage(usage: BufferUsage) -> types::GLenum {
        match usage {
            BufferUsage::Static => gl::STATIC_DRAW,
            BufferUsage::Dynamic => gl::DYNAMIC_DRAW,
            BufferUsage::Stream => gl::STREAM_DRAW,
        }
    }
}

impl Bindable for OglBuffer {
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.buffer_type, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.buffer_type, 0);
        }
    }
}

impl Buffer for OglBuffer {
    fn get_layout(&self) -> &Vec<BufferLayoutNode> {
        return &self.buffer_layout;
    }

    fn get_element_count(&self) -> usize {
        return self.element_count;
    }

    fn add_sub_data(&mut self, data: Vec<f32>, offset: usize) {
        self.bind();
        unsafe {
            gl::BufferSubData(
                self.buffer_type,
                offset as isize,
                (data.len() * size_of::<f32>()) as isize,
                data.as_ptr() as *const c_void,
            );
        }

        self.element_count = if offset == 0 {
            OglBuffer::calculate_element_count(data.len(), self.buffer_layout.clone(), BufferType::Vertex)
        } else {
            OglBuffer::calculate_element_count(offset + data.len(), self.buffer_layout.clone(), BufferType::Vertex)
        };
    }
}
