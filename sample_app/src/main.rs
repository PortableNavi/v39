use v39::{self, prelude::*};


struct App;
impl EventReceiver for App
{
    fn reset(&mut self) -> V39Result<()>
    {
        println!("Reset!");
        let handler = get_v39().event_handler();
        handler.queue_event(Event::new(CustomEvent::Ping, vec![]));
        Ok(())
    }

    fn tick(&mut self) -> V39Result<()>
    {
        let manager = get_v39().input_manager();

        if manager.is_down(input::V39Key::A)
        {
            println!("A was pressed");
        }

        Ok(())
    }

    fn dispatch_event(&mut self, event: Event) -> V39Result<()> 
    {
        let handler = get_v39().event_handler();

        if CustomEvent::Ping.eq(event.id)
        {
            //println!("Ping");
            handler.queue_event(Event::new(CustomEvent::Pong, vec![]));
        }

        else if CustomEvent::Pong.eq(event.id)
        {
            //println!("Pong");
            handler.queue_event(Event::new(CustomEvent::Ping, vec![]));
        }

        Ok(())
    }

    fn key_down(&mut self, key: input::V39Key) -> V39Result<()>
    {
        if key == input::V39Key::Q
        {
            get_v39().quit();
        }

        Ok(())
    }

    fn window_close(&mut self) -> V39Result<()>
    {
        get_v39().quit();
        Ok(())
    }

    fn quit(&mut self, reason: u32) -> V39Result<()> 
    {
        println!("Quitting because of reason: {reason}");
        Ok(())
    }

}


fn main() -> V39Result<()>
{
    let app = v39::init()?;

    app.event_handler().add_receiver(App);
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

