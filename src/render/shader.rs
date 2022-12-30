use nalgebra::{Matrix4, Vector3};

use crate::platform::opengl::shader::OglShader;

use super::buffer::Bindable;

pub trait Shader: Bindable {
    fn load_from_file(&mut self, file_path: &str);

    fn submit_uniform_vec3(&self, name: &str, value: Vector3<f32>);
    fn submit_uniform_mat4x4(&self, name: &str, data: Matrix4<f32>);
}

pub fn create_shader() -> Box<dyn Shader> {
    return Box::new(OglShader::new());
}

pub fn create_shader_from_file(file_path: &str) -> Box<dyn Shader> {
    return Box::new(OglShader::new_from_file(file_path));
}
