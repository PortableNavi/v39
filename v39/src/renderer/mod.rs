use crate::prelude::*;
use winit::window::Window;
use once_cell::sync::OnceCell;
use std::sync::Mutex;
use std::sync::Arc;
use raw_gl_context::{GlConfig, GlContext};
use glow::Context;
use raw_window_handle::HasRawWindowHandle;


pub(crate) const MAX_FRAMES_IN_FLIGHT: usize = 2;


static INSTANCE: OnceCell<Renderer> = OnceCell::new();


pub(crate) struct Renderer
{
    window: Arc<Window>,
    context: Mutex<Context>,
}


impl Renderer
{
    pub(crate) fn init(window: Arc<Window>) -> V39Result<&'static Self>
    {

        let raw_context = unsafe { GlContext::create(&window.as_ref(), GlConfig::default())}?;
        unsafe {raw_context.make_current()};

        let context = unsafe {Context::from_loader_function(|s| raw_context.get_proc_address(s))};

        let renderer = Renderer {window, context: Mutex::new(context)};
        
        if INSTANCE.set(renderer).is_err()
        {
            return Err(V39Error::Reinit("Renderer".into()));
        }

        info!("GL Renderer Initialized");
        Ok(INSTANCE.get().unwrap())
    }

    pub fn render(&self, func: impl FnOnce(&Context))
    {
        let mut ctx = &self.context.lock().unwrap();
        func(ctx);
    }

    pub(crate) fn destroy(&self)
    { 
        info!("GL Renderer Destroyed");
    }
}

unsafe impl Sync for Renderer {}
unsafe impl Send for Renderer {}
