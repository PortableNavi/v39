pub mod receiver;

#[allow(clippy::module_inception)]
pub mod event;

use once_cell::sync::OnceCell;
use std::sync::Mutex;
use crate::prelude::*;


static INSTANCE: OnceCell<EventHandler> = OnceCell::new();


pub(crate) struct EventHandler
{
    engine_events: Mutex<Vec<EngineEvent>>,
    events: Mutex<Vec<Event>>,
    receiver: Mutex<Vec<Box<dyn EventReceiver + Sync + Send>>>,
    locked_engine_events: Mutex<Vec<EngineEvent>>,
    locked_events: Mutex<Vec<Event>>,
    locked_receiver: Mutex<Vec<Box<dyn EventReceiver + Sync + Send>>>,
}


impl EventHandler
{
    pub(crate) fn init() -> V39Result<&'static Self>
    {
        if unsafe {INSTANCE.get()}.is_some()
        {
            return Err(V39Error::Reinit("EventHandler".into()));
        }

        let handler = EventHandler {
            engine_events: Mutex::new(vec![]),
            events: Mutex::new(vec![]),
            receiver: Mutex::new(vec![]),
            locked_engine_events: Mutex::new(vec![]),
            locked_events: Mutex::new(vec![]),
            locked_receiver: Mutex::new(vec![]),
        };

        
        INSTANCE.set(handler);
        Ok(INSTANCE.get().unwrap())
    }

    pub(crate) fn get() -> &'static Self
    {
        unsafe {INSTANCE.get().unwrap()}
    }

    pub(crate) fn snapchot_event_queue(&self)
    {
        if let (Ok(mut locked_events), Ok(mut events)) = (self.locked_events.lock(), self.events.lock())
        {
            *locked_events = events.drain(..).collect::<Vec<_>>();
        }
    }

    pub(crate) fn snapshot_engine_event_queue(&self, filter: impl Fn(&EngineEvent)->bool)
    {
        if let (Ok(mut locked_eevents), Ok(mut eevents)) = (self.locked_engine_events.lock(), self.engine_events.lock())
        {
            for e in eevents.drain(..).collect::<Vec<_>>()
            {
                if filter(&e) {locked_eevents.push(e)}
                else {eevents.push(e)}
            }
        }
 
    }

    pub(crate) fn record_event(&self, event: Event)
    {
        if let Ok(mut events) = self.events.lock()
        {
            events.push(event);
        }
    }

    pub(crate) fn record_engine_event(&self, event: EngineEvent)
    {
        if let Ok(mut eevents) = self.engine_events.lock()
        {
            eevents.push(event);
        }
    }

    pub(crate) fn snapchot_receiver_queue(&self)
    {
        if let (Ok(mut receivers), Ok(mut locked_receivers)) = (self.receiver.lock(), self.locked_receiver.lock())
        {
            *locked_receivers = receivers.drain(..).collect::<Vec<_>>();
        }
    }

    pub(crate) fn apply_receiver_snapshot(&self)
    {
        if let (Ok(mut receivers), Ok(mut locked_receivers)) = (self.receiver.lock(), self.locked_receiver.lock())
        {
            receivers.extend(locked_receivers.drain(..));
        }
    }

    pub(crate) fn record_receiver(&self, rec: Box<dyn EventReceiver + Sync + Send>)
    {
        if let Ok(mut receivers) = self.receiver.lock()
        {
            receivers.push(rec);
        }
    }

    pub(crate) fn fetch_event(&self) -> Option<Event>
    {
        if let Ok(mut events) = self.locked_events.lock()
        {
            return events.pop()
        }

        None
    }

    pub(crate) fn fetch_engine_event(&self) -> Option<EngineEvent>
    {
        if let Ok(mut eevents) = self.locked_engine_events.lock()
        {
            return eevents.pop()
        }

        None
    }

    pub(crate) fn fetch_event_snapshots(&self) -> Vec<Event>
    {
        
        if let Ok(mut events) = self.locked_events.lock()
        {
            return events.drain(..).collect::<Vec<_>>();
        }

        vec![]
    }

    pub(crate) fn fetch_engine_event_snapshots(&self) -> Vec<EngineEvent>
    {
        
        if let Ok(mut eevents) = self.locked_engine_events.lock()
        {
            return eevents.drain(..).collect::<Vec<_>>();
        }

        vec![]
    }

    pub(crate) fn foreach_receiver_snapshot(&self, mut f: impl FnMut(&mut Box<dyn EventReceiver + Send + Sync + 'static>) -> V39Result<()>)
    {
        if let Ok(mut recs) = self.locked_receiver.lock()
        {
            for rec in &mut *recs
            {
                if let Err(e) = f(rec)
                {
                    error!("Error while dispatching events: {e}");
                }
            }
        }
    }
}


#[derive(Debug, Clone)]
pub(crate) enum EngineEvent
{
    Reset, FrameBegin, FrameEnd,
    KeyDown(Option<input::V39Key>),
    KeyUp(Option<input::V39Key>),
    Tick(Option<f32>),
    FixedTick(Option<f32>),
    Quit(Option<u32>),
}


impl EngineEvent
{
    pub(crate) fn var_eq(&self, other: &Self) -> bool
    {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
