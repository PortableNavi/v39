use once_cell::sync::OnceCell;
use std::sync::Mutex;
use crate::interfaces::event_handler::EventHandlerInterface;
use crate::interfaces::input_manager::InputManagerInterface;
use crate::input::InputManager;
use crate::event::EngineEvent;
use crate::prelude::*;


static INSTANCE: OnceCell<App> = OnceCell::new();


pub struct App
{
    event_handler: EventHandlerInterface,
    input_manager: InputManagerInterface,
    quit: Mutex<bool>,
}


impl App
{
    pub(crate) fn init() -> V39Result<()>
    {
        let mut event_handler = EventHandlerInterface::new()?;
        let input_manager = InputManagerInterface::new()?;

        event_handler.add_receiver(input_manager.clone());

        let app = App {event_handler, input_manager, quit: Mutex::new(false)};

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

    #[inline]
    pub fn event_handler(&self) -> &EventHandlerInterface
    {
        &self.event_handler
    }

    #[inline]
    pub fn input_manager(&self) -> &InputManagerInterface
    {
        &self.input_manager
    }

    pub fn quit(&self)
    {
        *self.quit.lock().unwrap() = true;
    }

    pub fn run(&self) -> V39Result<()>
    {
        info!("Starting main loop");

        // Order of Events:
        // 1. Input
        // 2. FrameBegin
        // 3. Tick
        // 4. CustomEvents
        // 5. LateTick
        // 7. FrameEnd

        let handler = &self.event_handler;

        // Simulating a event loop...
        handler.fire_single_engine_event(EngineEvent::Reset);

        handler.fire_single_engine_event(EngineEvent::KeyDown(Some(input::V39Key::A)));
        handler.fire_single_engine_event(EngineEvent::FrameBegin);
        handler.fire_single_engine_event(EngineEvent::Tick(Some(0.0f32)));
        handler.fire_events();
        handler.fire_single_engine_event(EngineEvent::FrameEnd);

        for _ in 0..2
        {
            handler.fire_single_engine_event(EngineEvent::FrameBegin);
            handler.fire_single_engine_event(EngineEvent::Tick(Some(0.0f32)));
            handler.fire_events();
            handler.fire_single_engine_event(EngineEvent::FrameEnd);
        }

        handler.fire_single_engine_event(EngineEvent::KeyUp(Some(input::V39Key::A)));
        handler.fire_single_engine_event(EngineEvent::FrameBegin);
        handler.fire_single_engine_event(EngineEvent::Tick(Some(0.0f32)));
        handler.fire_events();
        handler.fire_single_engine_event(EngineEvent::FrameEnd);

        for _ in 0..2
        {
            handler.fire_single_engine_event(EngineEvent::FrameBegin);
            handler.fire_single_engine_event(EngineEvent::Tick(Some(0.0f32)));
            handler.fire_events();
            handler.fire_single_engine_event(EngineEvent::FrameEnd);
        }

        handler.fire_single_engine_event(EngineEvent::KeyDown(Some(input::V39Key::Q)));
        handler.fire_single_engine_event(EngineEvent::FrameBegin);
        handler.fire_single_engine_event(EngineEvent::Tick(Some(0.0f32)));
        handler.fire_events();
        handler.fire_single_engine_event(EngineEvent::FrameEnd);


        if *self.quit.lock().unwrap()
        {
            handler.fire_single_engine_event(EngineEvent::Quit(Some(0)));
        }

        Ok(())
    }
}
