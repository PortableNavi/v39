#![feature(portable_simd)]
use v39::prelude::*;


//#[macro_use]
//extern crate log;


#[derive(Default)]
struct App;


impl EventReceiver for App
{
    // Init stuff
    fn reset(&mut self) -> V39Result<()> 
    {
        get_v39().timer().set_target_fps(Some(60));

        let renderer = get_v39().renderer();

        let shader = Shader::new(&[
            ShaderSource::vertex(include_str!("../shaders/base.vert")),
            ShaderSource::fragment(include_str!("../shaders/base.frag")),
        ])?;

        let positions = [
            0.0f32,  0.5f32, 0.0f32,
            -0.5f32,  -0.5f32, 0.0f32,
            0.5f32,  -0.5f32, 0.0f32,
        ];

        let vbo = Vbo::new(&positions, glow::STATIC_DRAW)?;
        renderer.load_vbo(0, vbo);

        let vao = Vao::new(0)?;
        renderer.load_vao(0, vao);

        renderer.load_shader("base", shader);
        Ok(())
}

    // Quit the App on a window close event.
    fn window_close(&mut self) -> V39Result<()> 
    {
        get_v39().quit();
        Ok(())
    }

    // Quit the App if q is pressed.
    fn key_down(&mut self, key: input::V39Key) -> V39Result<()> 
    {
        if key == input::V39Key::Q {get_v39().quit();}
        Ok(())
    }

    // Each tick, render to the screen
    fn tick(&mut self, _delta: f32) -> V39Result<()> 
    {
        let renderer = get_v39().renderer();

        renderer.use_shader("base");
        renderer.use_vao(0);

        renderer.exec_gl(|gl| unsafe {
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
            Ok(())
        })
    }
}


fn main() -> V39Result<()>
{
    let props = InitProps {title: "Opengl Test".into(), ..InitProps::default()};
    let app = v39::init(&props)?;
    app.event_handler().add_receiver(App);
    app.run()
}

