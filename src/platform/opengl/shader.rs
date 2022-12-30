use std::{collections::HashMap, fs, ptr};

use std::ffi::CString;

use gl::types;

use crate::render::{buffer::Bindable, shader::Shader};

const VERTEX_KEY: &str = "vertex";
const FRAGMENT_KEY: &str = "fragment";

pub struct OglShader {
    id: u32,
}

#[allow(temporary_cstring_as_ptr)]
impl Shader for OglShader {
    fn load_from_file(&mut self, file_path: &str) {
        let file_content = fs::read_to_string(file_path).expect("Couldn't read shader file");
        let mut map: HashMap<&str, CString> = HashMap::new();
        let _split: Vec<&str> = file_content
            .split("#shader ")
            .filter_map(|x| match x.trim().is_empty() {
                true => None,
                false => {
                    let line_end = x.find("\n").unwrap();
                    let split = x.split_at(line_end);
                    map.insert(split.0, CString::new(split.1).unwrap());

                    return Some(x);
                }
            })
            .collect();

        let vertex_src = &map[VERTEX_KEY];
        let fragment_src = &map[FRAGMENT_KEY];

        unsafe {
            let vs_id = gl::CreateShader(gl::VERTEX_SHADER);
            let fs_id = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(vs_id, 1, &vertex_src.as_ptr(), ptr::null());
            gl::CompileShader(vs_id);
            OglShader::check_compile_errors(vs_id, "VERTEX");

            gl::ShaderSource(fs_id, 1, &fragment_src.as_ptr(), ptr::null());
            gl::CompileShader(fs_id);
            OglShader::check_compile_errors(fs_id, "FRAGMENT");

            gl::AttachShader(self.id, vs_id);
            gl::AttachShader(self.id, fs_id);
            gl::LinkProgram(self.id);
            OglShader::check_compile_errors(self.id, "PROGRAM");

            gl::DeleteShader(vs_id);
            gl::DeleteShader(fs_id);
        }
    }

    fn submit_uniform_mat4x4(&self, name: &str, data: nalgebra::Matrix4<f32>) {
        unsafe {
            let uniform_location = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr());
            gl::UniformMatrix4fv(uniform_location, 1, gl::FALSE, data.as_ptr());
        }
    }

    fn submit_uniform_vec3(&self, name: &str, data: nalgebra::Vector3<f32>) {
        unsafe {
            let uniform_location = gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr());
            gl::Uniform3fv(uniform_location, 1, data.as_ptr());
        }
    }
}

impl Bindable for OglShader {
    fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
}

impl OglShader {
    pub fn new() -> OglShader {
        let mut shader = OglShader { id: 0 };

        unsafe {
            shader.id = gl::CreateProgram();
        }

        return shader;
    }

    pub fn new_from_file(file_path: &str) -> OglShader {
        let mut shader = OglShader::new();
        shader.load_from_file(file_path);

        return shader;
    }

    fn check_compile_errors(id: u32, shader_type: &str) {
        let mut status: i32 = gl::TRUE as i32;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut status);
            if status == gl::FALSE as i32 {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut status);
                let buffer: Vec<u8> = vec![1; status as usize + 1];
                let error_string = CString::from_vec_unchecked(buffer);
                gl::GetShaderInfoLog(
                    id,
                    status,
                    ptr::null_mut(),
                    error_string.as_ptr() as *mut types::GLchar,
                );
                println!(
                    "[{}] Failed to compile shader with error\n{}",
                    shader_type,
                    error_string.to_str().unwrap()
                );
            }
        }
    }
}
