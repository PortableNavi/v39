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

        self.handler.snapchot_receiver_queue();
        self.handler.snapshot_engine_event_queue(|e| e.var_eq(&event_kind));
        
        let events = self.handler.fetch_engine_event_snapshots();
 
        for event in events
        {
            self.handler.foreach_receiver_snapshot(|rec|{
                match event
                {
                    EngineEvent::Reset => rec.reset(),
                    EngineEvent::Tick(_) => Ok(()),
                    EngineEvent::FixedTick(_) => Ok(()),
                    EngineEvent::Quit(_) => Ok(()), 
                }

            });
        }

        self.handler.apply_receiver_snapshot();
        trace!("Finished dispathing {event_kind:?} engine events");
        Ok(())
    }

    pub(crate) fn fire_events(&self) -> V39Result<()>
    {
        trace!("Begin dispatching events...");
        
        self.handler.snapchot_receiver_queue();
        self.handler.snapchot_event_queue();

        let mut events = self.handler.fetch_event_snapshots();

        while let Some(e) = events.pop()
        {
            self.handler.foreach_receiver_snapshot(|rec| rec.dispatch_event(e.to_owned()));
        }

        self.handler.apply_receiver_snapshot();

        trace!("Finished dispatching events");
        Ok(())
    }
}
