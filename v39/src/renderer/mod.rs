use crate::prelude::*;
use winit::window::Window;
use once_cell::sync::OnceCell;
use std::sync::Mutex;
use std::sync::Arc;


pub(crate) const MAX_FRAMES_IN_FLIGHT: usize = 2;


static INSTANCE: OnceCell<Renderer> = OnceCell::new();


pub(crate) struct Renderer
{
    window: Arc<Window>,
}


impl Renderer
{
    pub(crate) fn init(window: Arc<Window>) -> V39Result<&'static Self>
    {
        let renderer = Renderer {window};
        
        if INSTANCE.set(renderer).is_err()
        {
            return Err(V39Error::Reinit("Renderer".into()));
        }

        info!("GL Renderer Initialized");
        Ok(INSTANCE.get().unwrap())
    }

    pub(crate) fn destroy(&self)
    { 
        info!("GL Renderer Destroyed");
    }
}
