use once_cell::sync::OnceCell;
use std::sync::Mutex;
use crate::interfaces::event_handler::EventHandlerInterface;
use crate::prelude::*;


static INSTANCE: OnceCell<App> = OnceCell::new();


pub struct App
{
    event_handler: Mutex<EventHandlerInterface>,
}


impl App
{
    pub(crate) fn init() -> V39Result<()>
    {
        let event_handler = Mutex::new(EventHandlerInterface::new()?);

        let app = App {event_handler};

        if INSTANCE.set(app).is_err()
        {
            return Err(V39Error::Reinit("App".into()));
        }

        Ok(())
    }

    pub fn get() -> &'static App
    {
        INSTANCE.get().expect("App instance was not initialized")
    }

    pub fn event_handler(&self) -> &Mutex<EventHandlerInterface>
    {
        &self.event_handler
    }

    pub fn run(&self) -> V39Result<()>
    {
        if let Ok(mut handler) = self.event_handler().lock()
        {
            // Just do it a few times for testing
            for _ in 0..10
            {
                handler.fire_events();
            }
        }

        Ok(())
    }
}
