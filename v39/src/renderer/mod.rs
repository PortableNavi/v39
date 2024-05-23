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
    ctx: Mutex<Context>,
    rctx: Mutex<GlContext>,
}


impl Renderer
{
    pub(crate) fn init(window: Arc<Window>) -> V39Result<&'static Self>
    {

        let raw_context = unsafe { GlContext::create(&window.as_ref(), GlConfig::default())}?;

        unsafe {raw_context.make_current()};
        let context = unsafe {Context::from_loader_function(|s| raw_context.get_proc_address(s))};
        unsafe {raw_context.make_not_current()};

        let renderer = Renderer {window, ctx: Mutex::new(context), rctx: Mutex::new(raw_context)};
        
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

        unsafe {rctx.make_current()};

        func(&ctx)?;
        
        unsafe
        {
            rctx.swap_buffers();
            rctx.make_not_current();
        }

        Ok(())
    }

    pub(crate) fn destroy(&self)
    { 
        info!("GL Renderer Destroyed");
    }
}

unsafe impl Sync for Renderer {}
unsafe impl Send for Renderer {}
