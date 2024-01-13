use crate::event::{EventHandler, EngineEvent};
use crate::prelude::*;

pub struct EventHandlerInterface
{
    handler: &'static EventHandler,
}


impl EventHandlerInterface
{
    pub(crate) fn new() -> V39Result<Self>
    {
        let handler = EventHandler::init()?;
        info!("EventHandler Initialized");

        Ok(Self {handler})
    }

    pub fn add_receiver<T>(&self, receiver: T)
        where T: EventReceiver + Send + Sync + 'static
    {
        trace!("New EventReceiver registered");
        let receiver = Box::new(receiver);
        self.handler.record_receiver(receiver);
    }

    pub fn queue_event(&self, event: Event)
    {
        trace!("Event queued: {event:?}");
        self.handler.record_event(event);
    }

    pub(crate) fn queue_engine_event(&self, event: EngineEvent)
    {
        trace!("EngineEvent queued: {event:?}");
        self.handler.record_engine_event(event);
    }

    pub(crate) fn fire_engine_event(&self, event_kind: EngineEvent) -> V39Result<()>
    {
        trace!("Begin dispathing {event_kind:?} engine events...");
        self.event_dispatch_begin();

        self.handler.snapchot_receiver_queue();
        self.handler.snapshot_engine_event_queue(|e| e.var_eq(&event_kind));
        
        let events = self.handler.fetch_engine_event_snapshots();
 
        for event in events
        {
            self.handler.foreach_receiver_snapshot(|rec|{
                self.match_event(event.clone(), rec)
            });
        }

        self.handler.apply_receiver_snapshot();
        self.event_dispatch_end();
        trace!("Finished dispathing {event_kind:?} engine events");
        Ok(())
    }

    pub(crate) fn fire_events(&self) -> V39Result<()>
    {
        trace!("Begin dispatching events...");
        self.event_dispatch_begin();
        
        self.handler.snapchot_receiver_queue();
        self.handler.snapchot_event_queue();

        let mut events = self.handler.fetch_event_snapshots();

        while let Some(e) = events.pop()
        {
            self.handler.foreach_receiver_snapshot(|rec| rec.dispatch_event(e.to_owned()));
        }

        self.handler.apply_receiver_snapshot();
        
        self.event_dispatch_end();
        trace!("Finished dispatching events");
        Ok(())
    }

    pub(crate) fn fire_single_engine_event(&self, event: EngineEvent) -> V39Result<()>
    {
        trace!("Begin Single EngineEvent Dispatch of {event:?}");
        self.event_dispatch_begin();
        self.handler.snapchot_receiver_queue();

        self.handler.foreach_receiver_snapshot(|rec| {
            self.match_event(event.clone(), rec)
        });

        self.handler.apply_receiver_snapshot();
        self.event_dispatch_end();
        trace!("End Single Event Dispatch of {event:?}");

        Ok(())
    }

    pub(crate) fn event_dispatch_begin(&self)
    {
        get_v39().input_manager().event_begin();
    }

    pub(crate) fn event_dispatch_end(&self)
    {
        get_v39().input_manager().event_end();
    }

    fn match_event(&self,  event: EngineEvent, rec: &mut Box<dyn EventReceiver + Send + Sync>) -> V39Result<()>
    {
        match event
        {
            EngineEvent::Reset => rec.reset(),
            EngineEvent::KeyUp(Some(key)) => rec.key_up(key),
            EngineEvent::KeyDown(Some(key)) => rec.key_down(key),
            EngineEvent::FrameBegin => rec.frame_begin(),
            EngineEvent::FrameEnd => rec.frame_end(),
            EngineEvent::Tick(Some(_)) => rec.tick(),
            EngineEvent::Quit(Some(reason)) => rec.quit(reason),
            EngineEvent::WindowClose => rec.window_close(),
            
            _ => Ok(()),
        }
    }
}
