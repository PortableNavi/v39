use v39::{self, prelude::*};


struct App
{
    fps_cap: Option<u64>,
}

impl EventReceiver for App
{
    fn reset(&mut self) -> V39Result<()>
    {
        get_v39().timer().set_target_fps(self.fps_cap);
        Ok(())
    }

    fn key_down(&mut self, key: input::V39Key) -> V39Result<()>
    {
        match key
        {
            input::V39Key::Q => get_v39().quit(),
            input::V39Key::A => println!("A was pressed"),
            input::V39Key::F => {
                if self.fps_cap.is_some() {self.fps_cap = None}
                else {self.fps_cap = Some(60)}
                get_v39().timer().set_target_fps(self.fps_cap);
                println!("Fps cap set to: {:?}", self.fps_cap);
            },
            _ => {},
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
    
    let my_app = App {
        fps_cap: Some(60),
    };

    app.event_handler().add_receiver(my_app);
    app.run()
}


/*
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
*/
