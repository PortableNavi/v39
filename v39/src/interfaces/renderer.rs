use crate::prelude::*;
use crate::renderer::Renderer;
use std::sync::Arc;


#[derive(Clone)]
pub struct RendererInterface
{
    handle: &'static Renderer
}


impl RendererInterface
{
    pub(crate) fn new(window: Arc<winit::window::Window>) -> V39Result<Self>
    {
        Ok(Self {handle: Renderer::init(window)?})
    }

    pub fn exec_gl(&self, f: impl FnOnce(&glow::Context) -> V39Result<()>) -> V39Result<()>
    {
        self.handle.exec_gl(f)
    }

    pub(crate) fn destroy(&self)
    {
        self.handle.destroy();
    }
}


impl EventReceiver for RendererInterface
{

}

