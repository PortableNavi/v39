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

    pub fn load_shader(&self, id: &'static str, shader: Shader) -> bool
    {
        self.handle.load_shader(id, shader)        
    }

    pub fn unload_shader(&self, id: &'static str) -> bool
    {
        self.handle.unload_shader(id)
    }

    pub fn use_shader(&self, id: &'static str) -> bool
    {
        self.handle.use_shader(id)
    }

    pub fn clear_shader(&self)
    {
        let _ = self.handle.exec_gl(|gl| unsafe {
            gl.use_program(None);
            Ok(())
        });
    }
  
    pub(crate) fn finish_frame(&self)
    {
        self.handle.buffer_swap();
    }

    pub(crate) fn new_frame(&self)
    {
        self.handle.begin_frame();
    }

    pub(crate) fn destroy(&self)
    {
        self.handle.destroy();
    }
}


impl EventReceiver for RendererInterface
{
    fn window_resize(&mut self, size: (u32, u32)) -> V39Result<()> 
    {
        get_v39().renderer().exec_gl(|gl| unsafe {
            gl.viewport(0, 0, size.0 as i32, size.1 as i32);
            Ok(())
        })
    }

}

