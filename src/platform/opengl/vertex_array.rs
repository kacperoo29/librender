use std::{ffi::c_void, mem::size_of, ptr};

use gl::types;

use crate::render::buffer::{Bindable, Buffer, BufferDataType, BufferType, VertexArray};

// TODO: Check if buffers are ogl
pub struct OglVertexArray {
    id: u32,
    attribs: u32,
    vertex_buffers: Vec<Box<dyn Buffer>>,
    index_buffer: Option<Box<dyn Buffer>>,
    element_count: usize,
}

impl Bindable for OglVertexArray {
    fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) }
    }

    fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }
}

impl VertexArray for OglVertexArray {
    fn draw(&self) {
        self.bind();
        match &self.index_buffer {
            Some(ib) => {
                let data_type = match ib.get_layout().first() {
                    Some(node) => Self::translate_type(node.data_type),
                    None => gl::UNSIGNED_INT,
                };
                unsafe {
                    gl::DrawElements(
                        gl::TRIANGLES,
                        ib.get_element_count() as i32,
                        data_type,
                        ptr::null(),
                    )
                }
            }
            None => unsafe {
                gl::DrawArrays(gl::TRIANGLES, 0, self.element_count as i32);
            },
        }
    }

    fn add_buffer(&mut self, buffer: Box<dyn Buffer>, buffer_type: BufferType) {
        self.bind();
        buffer.bind();
        let mut pointer = 0;
        match buffer_type {
            BufferType::Vertex => {
                let layout = buffer.get_layout();
                let mut stride = 0;
                for node in layout {
                    stride += Self::get_type_size(node.data_type) * node.elements as usize;
                }
                self.element_count += buffer.get_element_count();
                for node in layout {
                    unsafe {
                        gl::EnableVertexArrayAttrib(self.id, self.attribs);
                        gl::VertexAttribPointer(
                            self.attribs,
                            node.elements as i32,
                            Self::translate_type(node.data_type),
                            if node.normalized { gl::TRUE } else { gl::FALSE },
                            stride as i32,
                            pointer as *const c_void,
                        );
                    }
                    self.attribs += 1;
                    pointer += Self::get_type_size(node.data_type) * node.elements as usize;
                }
                self.vertex_buffers.push(buffer);
            }
            BufferType::Index => {
                self.index_buffer = Some(buffer);
            }
        }
    }

    fn get_buffers(&self) -> &Vec<Box<dyn Buffer>> {
        return &self.vertex_buffers;
    }

    fn add_buffer_sub_data(&mut self, data: Vec<f32>, offset: usize, buffer_index: usize) {
        let old_buffer_size = self.vertex_buffers[buffer_index].get_element_count();
        self.vertex_buffers[buffer_index].add_sub_data(data, offset);
        self.element_count += self.vertex_buffers[buffer_index].get_element_count() - old_buffer_size;
    }
}

impl OglVertexArray {
    pub fn new() -> OglVertexArray {
        let mut array = OglVertexArray {
            id: 0,
            attribs: 0,
            vertex_buffers: Vec::new(),
            index_buffer: None,
            element_count: 0,
        };
        unsafe {
            gl::CreateVertexArrays(1, &mut array.id);
            gl::BindVertexArray(array.id);
        }

        return array;
    }

    pub fn translate_type(data_type: BufferDataType) -> types::GLenum {
        match data_type {
            BufferDataType::F32 => gl::FLOAT,
            BufferDataType::F64 => gl::DOUBLE,
            BufferDataType::I8 => gl::BYTE,
            BufferDataType::I16 => gl::SHORT,
            BufferDataType::I32 => gl::INT,
            BufferDataType::U8 => gl::UNSIGNED_BYTE,
            BufferDataType::U16 => gl::UNSIGNED_SHORT,
            BufferDataType::U32 => gl::UNSIGNED_INT,
        }
    }

    pub fn get_type_size(data_type: BufferDataType) -> usize {
        match data_type {
            BufferDataType::F32 => size_of::<f32>(),
            BufferDataType::F64 => size_of::<f64>(),
            BufferDataType::I8 => size_of::<i8>(),
            BufferDataType::I16 => size_of::<i16>(),
            BufferDataType::I32 => size_of::<i32>(),
            BufferDataType::U8 => size_of::<u8>(),
            BufferDataType::U16 => size_of::<u16>(),
            BufferDataType::U32 => size_of::<u32>(),
        }
    }
}
