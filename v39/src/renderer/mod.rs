use crate::prelude::*;
use winit::window::Window;
use once_cell::sync::OnceCell;
use std::sync::Mutex;
use std::sync::Arc;
use raw_gl_context::{GlConfig, GlContext};
use glow::Context;
use raw_window_handle::HasRawWindowHandle;
use std::collections::HashMap;
use std::rc::Rc;

mod ebo;
pub use ebo::Ebo;

mod texture;
pub use texture::{Texture, TexParam};

mod vbo;
pub use vbo::{Vbo, VboFormat};

mod vao;
pub use vao::Vao;

mod shader;
pub use shader::{Shader, ShaderSource, ShaderKind, UniformValue};

pub(crate) const MAX_FRAMES_IN_FLIGHT: usize = 2;


static INSTANCE: OnceCell<Renderer> = OnceCell::new();


pub(crate) struct Renderer
{
    window: Arc<Window>,
    ctx: Mutex<Context>,
    rctx: Mutex<GlContext>,
    shaders: Mutex<HashMap<usize, Rc<Shader>>>,
    vbos: Mutex<HashMap<usize, Rc<Vbo<{glow::FLOAT}, f32>>>>,
    vaos: Mutex<HashMap<usize, Rc<Vao>>>,
    ebos: Mutex<HashMap<usize, Rc<Ebo>>>,
    textures: Mutex<HashMap<usize, Rc<Texture>>>,
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
            ebos: Mutex::new(HashMap::default()),
            textures: Mutex::new(HashMap::default()),
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

    pub fn load_texture(&self, id: usize, texture: Texture) -> bool
    {
        let mut textures = self.textures.lock().unwrap();

        if textures.contains_key(&id)
        {
            return false;
        }

        textures.insert(id, Rc::new(texture));

        true
    }

    pub fn unload_texture(&self, id: usize) -> bool
    {
        self.textures.lock().unwrap().remove(&id).is_some()
    }

    pub fn use_texture(&self, id: usize, unit: u32, shader: usize, sampler_name: &str) -> bool
    {
        if let Some(texture) = self.textures.lock().unwrap().get(&id)
        {
            let prog = match self.get_shader(shader)
            {
                Some(shader) => shader.program(),
                None => return false,
            };

            let _ = self.exec_gl(|gl| unsafe {
                gl.active_texture(unit);
                gl.bind_texture(glow::TEXTURE_2D, Some(texture.image()));
                gl.use_program(Some(prog));
    
                if let Some(loc) = gl.get_uniform_location(prog, sampler_name)
                {
                    gl.uniform_1_i32(Some(&loc), 0);
                }

                else
                {
                    warn!("No sampler named {sampler_name:?} in shader {shader:?}");
                }

                Ok(())
            });
            
            return true;
        }

        false
    }
    
    pub fn clear_texture(&self)
    {
        let _ = self.exec_gl(|gl| unsafe {
            gl.bind_texture(glow::TEXTURE_2D, None);
            Ok(())
        });
    }

    pub fn get_texture(&self, id: usize) -> Option<Rc<Texture>>
    {
        self.textures.lock().unwrap().get(&id).cloned()
    }

    pub fn load_vbo(&self, id: usize, vbo: Vbo<{glow::FLOAT}, f32>) -> bool
    {
        let mut vbos = self.vbos.lock().unwrap();

        if vbos.contains_key(&id)
        {
            return false;
        }

        vbos.insert(id, Rc::new(vbo));

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

    pub fn load_ebo(&self, id: usize, ebo: Ebo) -> bool
    {
        let mut ebos = self.ebos.lock().unwrap();

        if ebos.contains_key(&id)
        {
            return false;
        }

        ebos.insert(id, Rc::new(ebo));

        true
    }

    pub fn unload_ebo(&self, id: usize) -> bool
    {
        self.ebos.lock().unwrap().remove(&id).is_some()
    }

    pub fn use_ebo(&self, id: usize) -> bool
    {
        if let Some(ebo) = self.ebos.lock().unwrap().get(&id)
        {
            let _ = self.exec_gl(|gl| unsafe {
                gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo.buffer()));
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

        vaos.insert(id, Rc::new(vao));

        true
    }

    pub fn unload_vao(&self, id: usize) -> bool
    {
        self.vaos.lock().unwrap().remove(&id).is_some()
    }

    pub fn use_vao(&self, id: usize) -> Option<u32>
    {
        if let Some(vao) = self.vaos.lock().unwrap().get(&id)
        {
            self.use_vbo(vao.vbo());
            self.use_ebo(vao.ebo());

            let _ = self.exec_gl(|gl| unsafe {
                gl.bind_vertex_array(Some(vao.buffer()));
                Ok(())
            });

            return Some(vao.count());
        }

        None
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

    pub fn clear_ebo(&self)
    {
        let _ = self.exec_gl(|gl| unsafe {
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None); 
            Ok(())
        });
    }

    pub fn load_shader(&self, id: usize, shader: Shader) -> bool
    {
        let mut shaders = self.shaders.lock().unwrap();

        if shaders.contains_key(&id)
        {
            return false;
        }

        shaders.insert(id, Rc::new(shader));

        true
    }

    pub fn unload_shader(&self, id: usize) -> bool
    {
        self.shaders.lock().unwrap().remove(&id).is_some()
    }

    pub fn use_shader(&self, id: usize) -> bool
    {
        if let Some(shader) = self.shaders.lock().unwrap().get(&id)
        {
            let _ = self.exec_gl(|gl| unsafe {
                gl.use_program(Some(shader.program()));
                Ok(())
            });
            
            return true;
        }

        false
    }

    pub fn set_shader_uniform(&self, id: usize, name: &str, val: UniformValue) -> bool
    {
        if let Some(shader) = self.shaders.lock().unwrap().get(&id)
        {
            return shader.set_uniform(name, val);
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

    pub fn is_shader_loaded(&self, id: usize) -> bool
    {
        self.shaders.lock().unwrap().contains_key(&id)
    }

    pub fn is_vbo_loaded(&self, id: usize) -> bool
    {
        self.vbos.lock().unwrap().contains_key(&id)
    }

    pub fn is_ebo_loaded(&self, id: usize) -> bool
    {
        self.ebos.lock().unwrap().contains_key(&id)
    }

    pub fn get_vbo(&self, id: usize) -> Option<Rc<Vbo<{glow::FLOAT}, f32>>>
    {
        self.vbos.lock().unwrap().get(&id).cloned()
    }

    pub fn get_ebo(&self, id: usize) -> Option<Rc<Ebo>>
    {
        self.ebos.lock().unwrap().get(&id).cloned()
    }

    pub fn get_vao(&self, id: usize) -> Option<Rc<Vao>>
    {
        self.vaos.lock().unwrap().get(&id).cloned()
    }

    pub fn get_shader(&self, id: usize) -> Option<Rc<Shader>>
    {
        self.shaders.lock().unwrap().get(&id).cloned()
    }

    pub(crate) fn destroy(&self)
    {
        self.clear_vbo();
        self.clear_ebo();
        self.clear_vao();
        self.clear_shader();

        self.vbos.lock().unwrap().clear();
        self.ebos.lock().unwrap().clear();
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


pub(crate) unsafe fn to_bytes<T>(slice: &[T]) -> &[u8]
{
    core::slice::from_raw_parts(
        slice.as_ptr() as *const u8,
        std::mem::size_of_val(slice),
    )
}
