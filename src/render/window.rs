use std::ffi::c_void;

use crate::{platform::glfw as trash_glfw, event::Event};

pub trait Window {
    fn close(&mut self);
    fn get_size(&self) -> (f32, f32);
    fn get_events(&mut self) -> Vec<Event>;
    fn get_proc_addr(&mut self, procname: &str) -> *const c_void;
    fn should_close(&self) -> bool;
    fn update(&mut self);
    fn set_title(&mut self, title: &str);
    fn toggle_mouse_grab(&mut self);
}

pub fn create_window() -> Box<dyn Window> {
    return trash_glfw::GlfwWindow::new();
}
