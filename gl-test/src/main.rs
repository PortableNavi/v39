use v39::prelude::*;


struct App;
impl EventReceiver for App
{
    fn window_close(&mut self) -> V39Result<()> 
    {
        get_v39().quit();
        Ok(())
    }

    fn tick(&mut self, _delta: f32) -> V39Result<()> 
    {
        get_v39().renderer().render(|gl|unsafe {
            gl.clear_color(0.5, 0.5, 1.0, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
        });

        Ok(())
    }
}


fn main() -> V39Result<()>
{
    let app = v39::init()?;
    app.event_handler().add_receiver(App);
    app.run()
}
