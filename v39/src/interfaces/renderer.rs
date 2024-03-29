use crate::prelude::*;
use crate::renderer::Renderer;


#[derive(Clone)]
pub struct RendererInterface
{
    handle: &'static Renderer
}


impl RendererInterface
{
    pub(crate) fn new(window: &winit::window::Window) -> V39Result<Self>
    {
        Ok(Self {handle: Renderer::init(window)?})
    }
}


impl EventReceiver for RendererInterface
{

}

