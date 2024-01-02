use v39::{self, prelude::*};


struct App;
impl EventReceiver for App
{
    fn dispatch_event(&mut self, event: Event, handler: &mut EventHandlerInterface) -> V39Result<()> 
    {
        if CustomEvent::Ping.eq(event.id)
        {
            println!("Ping");
            handler.queue_event(Event::new(CustomEvent::Pong, vec![]));
        }

        else if CustomEvent::Pong.eq(event.id)
        {
            println!("Pong");
            handler.queue_event(Event::new(CustomEvent::Ping, vec![]));
        }

        Ok(())
    }
}


fn main() -> V39Result<()>
{
    let app = v39::init()?;

    let event = Event::new(CustomEvent::Ping, vec![]);
    
    if let Ok(mut event_handler) = app.event_handler().lock()
    {
        event_handler.add_receiver(App);
        event_handler.queue_event(event);
    }

    app.run()
}


#[repr(u32)]
#[derive(Copy, Clone, Debug)]
enum CustomEvent
{
    Ping,
    Pong,
}


impl From<CustomEvent> for u32
{
    fn from(value: CustomEvent) -> Self 
    {
        value as u32
    }
}


impl CustomEvent
{
    fn eq(&self, other: u32) -> bool
    {
        *self as u32 == other
    }
}

