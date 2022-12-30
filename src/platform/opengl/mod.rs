extern crate gl;

pub mod buffer;
pub mod shader;
pub mod texture;
pub mod vertex_array;

use crate::render::mesh::Drawable;
use crate::render::render_api::RenderAPI;
use crate::render::window::Window;

pub struct OglAPI {
    initalized: bool,
}

impl RenderAPI for OglAPI {
    fn init(&self, window: &mut Box<dyn Window>) {
        gl::load_with(|s| window.get_proc_addr(s));
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            
            gl::Enable(gl::DEPTH_TEST);

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    fn clear(&self) {
        if !self.initalized {
            panic!("Ogl API was not initalized!");
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    fn begin(&self) {
        if !self.initalized {
            panic!("Ogl API was not initalized!");
        }

        self.clear();
    }

    fn end(&self) {
        if !self.initalized {
            panic!("Ogl API was not initalized!");
        }

        unsafe {
            gl::Flush();
        }
    }

    fn submit(&self, drawable: &mut Box<dyn Drawable>) {
        if !self.initalized {
            panic!("Ogl API was not initalized!");
        }

        drawable.draw();
    }

    fn disable_align_restrictions(&self) {
        if !self.initalized {
            panic!("Ogl API was not initalized!");
        }

        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        }
    }

    fn enable_align_restrictions(&self) {
        if !self.initalized {
            panic!("Ogl API was not initalized!");
        }

        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 4);
        }
    }

}

impl OglAPI {
    pub fn new(window: &mut Box<dyn Window>) -> OglAPI {
        let api = OglAPI { initalized: true };
        api.init(window);

        return api;
    }
}
