use crate::event::{EventHandler, EngineEvent};
use crate::prelude::*;

pub struct EventHandlerInterface
{
    handler: &'static mut EventHandler,
}


impl EventHandlerInterface
{
    pub(crate) fn new() -> V39Result<Self>
    {
        let handler = EventHandler::init()?;
        info!("EventHandler Initialized");

        Ok(Self {handler})
    }

    pub fn add_receiver<T>(&mut self, receiver: T)
        where T: EventReceiver + Send + Sync + 'static
    {
        trace!("New EventReceiver registered");
        let receiver = Box::new(receiver);
        self.handler.receiver.push(receiver);
    }

    pub fn queue_event(&mut self, event: Event)
    {
        trace!("Event queued: {event:?}");
        self.handler.events.push(event);
    }

    pub(crate) fn queue_engine_event(&mut self, event: EngineEvent)
    {
        trace!("EngineEvent queued: {event:?}");
        self.handler.engine_events.push(event);
    }

    pub(crate) fn fire_engine_event(&mut self, event_kind: EngineEvent) -> V39Result<()>
    {
        trace!("Begin dispathing {event_kind:?} engine events...");

        let events = self.handler.engine_events.drain(..).collect::<Vec<_>>();

        for event in events
        {
            if event.var_eq(&event_kind)
            {
                for handler in &mut self.handler.receiver
                {
                    match event
                    {
                        EngineEvent::Reset => handler.reset(&mut EventHandlerInterface{handler: EventHandler::get()})?,
                        EngineEvent::Tick(_) => (),
                        EngineEvent::FixedTick(_) => (),
                        EngineEvent::Quit(_) => (), 
                    }
                }
            }

            else
            {
                self.handler.engine_events.push(event);
            }
        }

        trace!("Finished dispathing {event_kind:?} engine events");
        Ok(())
    }

    pub(crate) fn fire_events(&mut self) -> V39Result<()>
    {
        trace!("Begin dispatching events...");

        for i in 0..self.handler.receiver.len()
        {
           for event in self.handler.events.drain(..)
           {
               self.handler.receiver[i].dispatch_event(event.clone(),
                   // This is ok because this reference does not live longer than this function
                   // call and the call to fire_events(..) is behind a mutex.
                   // Since fire_events() does not do anything asynchronos while a second mutable
                   // reference to the EventHandler exists it should be safe...
                   &mut EventHandlerInterface{handler: EventHandler::get()}
               )?;
           }
        }

        trace!("Finished dispatching events");
        Ok(())
    }
}
