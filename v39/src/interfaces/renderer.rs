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

    pub fn load_vbo(&self, id: usize, vbo: Vbo<{glow::FLOAT}, f32>) -> bool
    {
        self.handle.load_vbo(id, vbo)        
    }

    pub fn unload_vbo(&self, id: usize) -> bool
    {
        self.handle.unload_vbo(id)
    }

    pub fn use_vbo(&self, id: usize) -> bool
    {
        self.handle.use_vbo(id)
    }

    pub fn clear_vbo(&self)
    {
        self.handle.clear_vbo();
    }

    pub fn load_vao(&self, id: usize, vao: Vao) -> bool
    {
        self.handle.load_vao(id, vao)
    }

    pub fn unload_vao(&self, id: usize) -> bool
    {
        self.handle.unload_vao(id)
    }

    pub fn use_vao(&self, id: usize) -> bool
    {
        self.handle.use_vao(id)
    }

    pub fn clear_vao(&self)
    {
        self.handle.clear_vao()
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
        self.handle.clear_shader();
    }

    pub fn is_shader_loaded(&self, id: &'static str) -> bool
    {
        self.handle.is_shader_loaded(id)
    }

    pub fn is_vbo_loaded(&self, id: usize) -> bool
    {
        self.handle.is_vbo_loaded(id)
    }

    pub(crate) fn get_vbo(&self, id: usize) -> Option<Vbo<{glow::FLOAT}, f32>>
    {
        self.handle.get_vbo(id)
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

