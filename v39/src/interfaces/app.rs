use once_cell::sync::OnceCell;

use std::sync::{Mutex, Arc};
//use std::sync::atomic::{Ordering, AtomicPtr};

use crate::interfaces::event_handler::EventHandlerInterface;
use crate::interfaces::input_manager::InputManagerInterface;
use crate::interfaces::timer::TimerInterface;
use crate::interfaces::renderer::RendererInterface;
use crate::input::InputManager;
use crate::event::EngineEvent;
use crate::prelude::*;

use winit::{
    event::{Event, WindowEvent, KeyEvent, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    window::Window,
    keyboard::PhysicalKey,
};


static INSTANCE: OnceCell<App> = OnceCell::new();


pub struct App
{
    event_handler: EventHandlerInterface,
    input_manager: InputManagerInterface,
    timer: TimerInterface,
    renderer: RendererInterface,

    window: Arc<Window>,
    event_loop: Mutex<Option<EventLoop<()>>>,

    quit: Mutex<bool>,
}


unsafe impl Sync for App {}
unsafe impl Send for App {}


impl App
{
    pub(crate) fn init() -> V39Result<()>
    {
        let mut event_handler = EventHandlerInterface::new()?;
        let input_manager = InputManagerInterface::new()?;
        let timer = TimerInterface::new()?;

        //TODO: Wrap the winit errors...
        let event_loop = EventLoop::new().unwrap();

        let window = WindowBuilder::new()
            .with_title("V39 App")
            .build(&event_loop).unwrap();

        event_loop.set_control_flow(ControlFlow::Wait);
        let event_loop = Mutex::new(Some(event_loop));

        let window = Arc::new(window);
        let renderer = RendererInterface::new(window.clone())?;

        event_handler.add_receiver(input_manager.clone());
        event_handler.add_receiver(timer.clone());
        event_handler.add_receiver(renderer.clone());

        let app = App {event_handler, input_manager, timer, window, event_loop, renderer, quit: Mutex::new(false)};

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

    #[inline]
    pub fn timer(&self) -> &TimerInterface
    {
        &self.timer
    }

    #[inline]
    pub fn renderer(&self) -> &RendererInterface
    {
        &self.renderer
    }

    pub fn quit(&self)
    {
        *self.quit.lock().unwrap() = true;
    }

    pub fn run(&self) -> V39Result<()>
    {
        // :)
        let event_loop = self.event_loop
            .lock().unwrap()
            .take().unwrap();

        std::thread::scope(|s| {
            s.spawn(|| self.main_loop());

            let event_handler = self.event_handler();

            event_loop.run(move |e, elwt| {
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
                            WindowEvent::CloseRequested => event_handler.queue_engine_event(EngineEvent::WindowClose),
                            WindowEvent::Focused(true) => event_handler.queue_engine_event(EngineEvent::WindowFocus),
                            WindowEvent::Focused(false) => event_handler.queue_engine_event(EngineEvent::WindowUnfocus),
                            WindowEvent::Resized(size) => event_handler.queue_engine_event(EngineEvent::WindowResize(Some((size.width, size.height)))),
                            _ => {},
                        }
                    },

                    _ => () 
                }

                if let Ok(quit) = self.quit.lock()
                {
                    if *quit {elwt.exit();}
                }
            });

            event_handler.fire_single_engine_event(EngineEvent::Quit(Some(0)));
        });

        self.destroy();

        Ok(())
    }

    fn main_loop(&self)
    {
        let event_handler = self.event_handler();

        event_handler.fire_single_engine_event(EngineEvent::Reset);

        loop
        {
            if let Ok(quit) = self.quit.lock()
            {
                if *quit {break}
            }

            event_handler.fire_engine_event(EngineEvent::KeyDown(None));
            event_handler.fire_engine_event(EngineEvent::KeyUp(None));
            event_handler.fire_single_engine_event(EngineEvent::FrameBegin);
            event_handler.fire_single_engine_event(EngineEvent::Tick(Some(self.timer.delta_time().as_secs_f32())));
            event_handler.fire_engine_event(EngineEvent::WindowClose);
            event_handler.fire_events();
            event_handler.fire_single_engine_event(EngineEvent::FrameEnd);

            self.timer.pad_frame_time();
        }
    }

    fn destroy(&self)
    {
        self.renderer.destroy();
        info!("App Destroyed");
    }
}

