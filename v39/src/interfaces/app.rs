use once_cell::sync::OnceCell;
use std::sync::Mutex;
use crate::interfaces::event_handler::EventHandlerInterface;
use crate::interfaces::input_manager::InputManagerInterface;
use crate::input::InputManager;
use crate::event::EngineEvent;
use crate::prelude::*;

use winit::{
    event::{Event, WindowEvent, KeyEvent, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    keyboard::PhysicalKey,
};

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
        //TODO: Wrap the winit errors...
        let winit_event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new().build(&winit_event_loop).unwrap();
        winit_event_loop.set_control_flow(ControlFlow::Poll);

        let event_handler = self.event_handler();
        self.event_handler.fire_single_engine_event(EngineEvent::Reset);

        winit_event_loop.run(move |e, elwt| {
            match e
            {
                Event::WindowEvent {event: WindowEvent::KeyboardInput {event, ..}, ..} => {
                    if !event.repeat && let PhysicalKey::Code(key) = event.physical_key
                    {
                        let key = Some(input::translate::winit_key_to_v39_key(&key));

                        match event.state
                        {
                            ElementState::Pressed => event_handler.queue_engine_event(EngineEvent::KeyDown(key)),
                            ElementState::Released => event_handler.queue_engine_event(EngineEvent::KeyUp(key)),
                        }
                    }
                },

                Event::WindowEvent {event, ..} => {
                    match event
                    {
                        WindowEvent::CloseRequested => event_handler.fire_single_engine_event(EngineEvent::WindowClose),
                        _ => Ok(()),
                    };
                },

                _ => ()   
            }

            if let Ok(quit) = self.quit.lock()
            {
                if *quit {elwt.exit();}
            }

            event_handler.fire_engine_event(EngineEvent::KeyDown(None));
            event_handler.fire_engine_event(EngineEvent::KeyUp(None));
            event_handler.fire_single_engine_event(EngineEvent::FrameBegin);
            event_handler.fire_single_engine_event(EngineEvent::Tick(Some(0.0f32)));
            event_handler.fire_events();
            event_handler.fire_single_engine_event(EngineEvent::FrameEnd);
        });

        event_handler.fire_single_engine_event(EngineEvent::Quit(Some(0)));

        Ok(())
    }
}
