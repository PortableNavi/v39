pub mod receiver;

#[allow(clippy::module_inception)]
pub mod event;

use once_cell::sync::OnceCell;
use std::sync::Mutex;
use crate::prelude::*;


static mut INSTANCE: OnceCell<EventHandler> = OnceCell::new();


pub(crate) struct EventHandler
{
    pub(crate) engine_events: Vec<EngineEvent>,
    pub(crate) events: Vec<Event>,
    pub(crate) receiver: Vec<Box<dyn EventReceiver + Sync + Send>>,
}


impl EventHandler
{
    pub(crate) fn init() -> V39Result<&'static mut Self>
    {
        if unsafe {INSTANCE.get()}.is_some()
        {
            return Err(V39Error::Reinit("EventHandler".into()));
        }

        let handler = EventHandler {
            engine_events: vec![],
            events: vec![],
            receiver: vec![],
        };

        unsafe 
        {
            INSTANCE.set(handler);
            Ok(INSTANCE.get_mut().unwrap())
        }
    }

    pub(crate) fn get() -> &'static mut Self
    {
        unsafe {INSTANCE.get_mut().unwrap()}
    }
}


#[derive(Debug, Clone)]
pub enum EngineEvent
{
    Reset,
    Tick(Option<f32>),
    FixedTick(Option<f32>),
    Quit(Option<u32>),
}
