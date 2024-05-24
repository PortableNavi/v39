use crate::prelude::*;
use winit::window::Window;
use once_cell::sync::OnceCell;
use std::sync::Mutex;
use std::sync::Arc;
use raw_gl_context::{GlConfig, GlContext};
use glow::Context;
use raw_window_handle::HasRawWindowHandle;
use std::collections::HashMap;

mod vbo;
pub use vbo::Vbo;

mod vao;
pub use vao::Vao;

mod shader;
pub use shader::{Shader, ShaderSource, ShaderKind};

pub(crate) const MAX_FRAMES_IN_FLIGHT: usize = 2;


static INSTANCE: OnceCell<Renderer> = OnceCell::new();


pub(crate) struct Renderer
{
    window: Arc<Window>,
    ctx: Mutex<Context>,
    rctx: Mutex<GlContext>,
    shaders: Mutex<HashMap<&'static str, Shader>>,
    vbos: Mutex<HashMap<usize, Vbo<{glow::FLOAT}, f32>>>,
    vaos: Mutex<HashMap<usize, Vao>>,
}


impl Renderer
{
    pub(crate) fn init(window: Arc<Window>) -> V39Result<&'static Self>
    {

        let raw_context = unsafe { GlContext::create(&window.as_ref(), GlConfig::default())}?;

        unsafe {raw_context.make_current()};
        let context = unsafe {Context::from_loader_function(|s| raw_context.get_proc_address(s))};
        unsafe {raw_context.make_not_current()};

        let renderer = Renderer {
            window, 
            ctx: Mutex::new(context),
            rctx: Mutex::new(raw_context),
            shaders: Mutex::new(HashMap::default()),
            vbos: Mutex::new(HashMap::default()),
            vaos: Mutex::new(HashMap::default()),
        };
        
        if INSTANCE.set(renderer).is_err()
        {
            return Err(V39Error::Reinit("Renderer".into()));
        }

        info!("GL Renderer Initialized");
        Ok(INSTANCE.get().unwrap())
    }

    pub fn exec_gl(&self, func: impl FnOnce(&Context) -> V39Result<()>) -> V39Result<()>
    {
        let ctx = self.ctx.lock().unwrap();
        let rctx = self.rctx.lock().unwrap();
        unsafe {rctx.make_current()}
        let result = func(&ctx);
        unsafe {rctx.make_not_current()};
        result
    }

    pub fn load_vbo(&self, id: usize, vbo: Vbo<{glow::FLOAT}, f32>) -> bool
    {
        let mut vbos = self.vbos.lock().unwrap();

        if vbos.contains_key(&id)
        {
            return false;
        }

        vbos.insert(id, vbo);

        true
    }

    pub fn unload_vbo(&self, id: usize) -> bool
    {
        self.vbos.lock().unwrap().remove(&id).is_some()
    }

    pub fn use_vbo(&self, id: usize) -> bool
    {
        if let Some(vbo) = self.vbos.lock().unwrap().get(&id)
        {
            let _ = self.exec_gl(|gl| unsafe {
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo.buffer()));
                Ok(())
            });
            
            return true;
        }

        false
    }

    pub fn load_vao(&self, id: usize, vao: Vao) -> bool
    {
        let mut vaos = self.vaos.lock().unwrap();

        if vaos.contains_key(&id)
        {
            return false;
        }

        vaos.insert(id, vao);

        true
    }

    pub fn unload_vao(&self, id: usize) -> bool
    {
        self.vaos.lock().unwrap().remove(&id).is_some()
    }

    pub fn use_vao(&self, id: usize) -> bool
    {
        if let Some(vao) = self.vaos.lock().unwrap().get(&id)
        {
            self.use_vbo(vao.vbo());

            let _ = self.exec_gl(|gl| unsafe {
                gl.bind_vertex_array(Some(vao.buffer()));
                Ok(())
            });
            
            return true;
        }

        false
    }

    pub fn clear_vao(&self)
    {
        let _ = self.exec_gl(|gl| unsafe {
            gl.bind_vertex_array(None); 
            Ok(())
        });
    }

    pub fn clear_vbo(&self)
    {
        let _ = self.exec_gl(|gl| unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, None); 
            Ok(())
        });
    }

    pub fn load_shader(&self, id: &'static str, shader: Shader) -> bool
    {
        let mut shaders = self.shaders.lock().unwrap();

        if shaders.contains_key(id)
        {
            return false;
        }

        shaders.insert(id, shader);

        true
    }

    pub fn unload_shader(&self, id: &'static str) -> bool
    {
        self.shaders.lock().unwrap().remove(id).is_some()
    }

    pub fn use_shader(&self, id: &'static str) -> bool
    {
        if let Some(shader) = self.shaders.lock().unwrap().get(id)
        {
            let _ = self.exec_gl(|gl| unsafe {
                gl.use_program(Some(shader.program()));
                Ok(())
            });
            
            return true;
        }

        false
    }

    pub fn clear_shader(&self)
    {
        let _ = self.exec_gl(|gl| unsafe {
            gl.use_program(None); 
            Ok(())
        });
    }

    pub fn is_shader_loaded(&self, id: &'static str) -> bool
    {
        self.shaders.lock().unwrap().contains_key(id)
    }

    pub fn is_vbo_loaded(&self, id: usize) -> bool
    {
        self.vbos.lock().unwrap().contains_key(&id)
    }

    pub(crate) fn get_vbo(&self, id: usize) -> Option<Vbo<{glow::FLOAT}, f32>>
    {
        self.vbos.lock().unwrap().get(&id).cloned()
    }

    pub(crate) fn destroy(&self)
    {
        self.clear_vbo();
        self.clear_shader();

        self.vbos.lock().unwrap().clear();
        self.shaders.lock().unwrap().clear();

        info!("GL Renderer Destroyed");
    }

    pub(crate) fn buffer_swap(&self)
    {
        self.rctx.lock().unwrap().swap_buffers();
    }

    pub(crate) fn begin_frame(&self)
    {
        let _ = self.exec_gl(|gl| unsafe {
            gl.clear_color(0.5, 0.5, 1.0, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
            Ok(())
        });
    }
}

unsafe impl Sync for Renderer {}
unsafe impl Send for Renderer {}
