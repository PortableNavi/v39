use crate::prelude::*;
use crate::renderer::Renderer;
use std::sync::Arc;
use std::rc::Rc;


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

    pub fn load_model(&self, model: Model) -> Option<ModelId>
    {
        self.handle.load_model(model)
    }

    pub fn unload_model(&self, id: ModelId) -> bool
    {
        self.handle.unload_model(id)
    }

    pub fn use_model(&self, id: ModelId, cam: &Camera) -> Option<i32>
    {
        self.handle.use_model(id, cam).map(|id| id as i32)
    }

    pub fn draw_model(&self, id: ModelId, cam: &Camera) -> bool
    {
        self.handle.draw_model(id, cam)
    }

    pub fn get_model(&self, id: ModelId) -> Option<Rc<Model>>
    {
        self.handle.get_model(id)
    }

    pub fn exec_gl(&self, f: impl FnOnce(&glow::Context) -> V39Result<()>) -> V39Result<()>
    {
        self.handle.exec_gl(f)
    }

    pub fn load_texture(&self, texture: Texture) -> TextureId
    {
        let id = TextureId::new();
        self.handle.load_texture(id, texture);
        id
    }

    pub fn unload_texture(&self, id: TextureId) -> bool
    {
        self.handle.unload_texture(id)
    }

    pub fn use_texture(&self, id: TextureId, unit: u32, shader: ShaderId, sampler_name: &str) -> bool
    {
        self.handle.use_texture(id, unit, shader, sampler_name)
    }
    
    pub fn clear_texture(&self)
    {
        self.handle.clear_texture()
    }

    pub fn get_texture(&self, id: TextureId) -> Option<Rc<Texture>>
    {
        self.handle.get_texture(id)
    }

    pub fn load_vbo(&self, id: ModelId, vbo: Vbo<{glow::FLOAT}, f32>) -> bool
    {
        self.handle.load_vbo(id, vbo)        
    }

    pub fn unload_vbo(&self, id: ModelId) -> bool
    {
        self.handle.unload_vbo(id)
    }

    pub fn use_vbo(&self, id: ModelId) -> bool
    {
        self.handle.use_vbo(id)
    }

    pub fn clear_vbo(&self)
    {
        self.handle.clear_vbo();
    }

    pub fn load_ebo(&self, id: ModelId, ebo: Ebo) -> bool
    {
        self.handle.load_ebo(id, ebo)
    }

    pub fn unload_ebo(&self, id: ModelId) -> bool
    {
        self.handle.unload_ebo(id)
    }

    pub fn use_ebo(&self, id: ModelId) -> bool
    {
        self.handle.use_ebo(id)
    }

    pub fn clear_ebo(&self)
    {
        self.handle.clear_ebo()
    }

    pub fn load_vao(&self, id: ModelId, vao: Vao) -> bool
    {
        self.handle.load_vao(id, vao)
    }

    pub fn unload_vao(&self, id: ModelId) -> bool
    {
        self.handle.unload_vao(id)
    }

    pub fn use_vao(&self, id: ModelId) -> Option<i32>
    {
        self.handle.use_vao(id).map(|v| v as i32)
    }

    pub fn clear_vao(&self)
    {
        self.handle.clear_vao()
    }

    pub fn load_shader(&self, shader: Shader) -> ShaderId
    {
        let id = ShaderId::new();
        self.handle.load_shader(id, shader);
        id
    }

    pub fn unload_shader(&self, id: ShaderId) -> bool
    {
        self.handle.unload_shader(id)
    }

    pub fn use_shader(&self, id: ShaderId) -> bool
    {
        self.handle.use_shader(id)
    }

    pub fn set_shader_uniform(&self, id: ShaderId, name: &str, val: UniformValue) -> bool
    {
        self.handle.set_shader_uniform(id, name, val)
    }

    pub fn clear_shader(&self)
    {
        self.handle.clear_shader();
    }

    pub fn is_shader_loaded(&self, id: ShaderId) -> bool
    {
        self.handle.is_shader_loaded(id)
    }

    pub fn is_vbo_loaded(&self, id: ModelId) -> bool
    {
        self.handle.is_vbo_loaded(id)
    }

    pub fn is_ebo_loaded(&self, id: ModelId) -> bool
    {
        self.handle.is_ebo_loaded(id)
    }

    pub fn get_vbo(&self, id: ModelId) -> Option<Rc<Vbo<{glow::FLOAT}, f32>>>
    {
        self.handle.get_vbo(id)
    }

    pub fn get_ebo(&self, id: ModelId) -> Option<Rc<Ebo>>
    {
        self.handle.get_ebo(id)
    }

    pub fn get_vao(&self, id: ModelId) -> Option<Rc<Vao>>
    {
        self.handle.get_vao(id)
    }

    pub fn get_shader(&self, id: ShaderId) -> Option<Rc<Shader>>
    {
        self.handle.get_shader(id)
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

