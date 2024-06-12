use v39::prelude::*;
use v39::math::*;

mod camera;
mod scene;


#[derive(Default)]
struct App
{
    model: ModelId,
}


impl EventReceiver for App
{
    // Init stuff
    fn reset(&mut self) -> V39Result<()> 
    {
        get_v39().timer().set_target_fps(Some(60));

        let shader = scene::load_shaders()?;
        let texture = scene::load_textures()?;

        if let Some(id) = scene::load_model(shader, texture)?
        {
            self.model = id;
        }

        Ok(())  
    }

    // Quit the App on a window close event.
    fn window_close(&mut self) -> V39Result<()> 
    {
        get_v39().quit(); 
        Ok(())
    }

    // Each tick, render to the screen
    fn tick(&mut self, delta: f32) -> V39Result<()> 
    {
        let renderer = get_v39().renderer();

        if let Some(model) = renderer.get_model(self.model)
        {
            model.transform(|mat| {glm::rotate_y(mat, 0.7*delta)});
        }

        renderer.draw_model(self.model);

        Ok(())
    }
}


fn main() -> V39Result<()>
{
    let props = InitProps {title: "GL Test".into(), ..InitProps::default()};
    let app = v39::init(&props)?;

    app.event_handler().add_receiver(App::default());
    app.event_handler().add_receiver(camera::MainCamera::default());
    
    app.run()
}
