use crate::{platform::opengl::OglAPI, render::window::Window};

use super::mesh::{Drawable};

pub trait RenderAPI: Send + Sync {
    fn init(&self, window: &mut Box<dyn Window>);
    fn clear(&self);
    
    fn begin(&self);
    fn end(&self);
    fn submit(&self, drawable: &mut Box<dyn Drawable>);

    fn enable_align_restrictions(&self);
    fn disable_align_restrictions(&self);
}

pub fn create_api(window: &mut Box<dyn Window>) -> Box<dyn RenderAPI> {
    return Box::new(OglAPI::new(window));
}
