use v39::prelude::*;


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
            ShaderSource::vertex(include_str!("../shaders/default.vert")),
            ShaderSource::fragment(include_str!("../shaders/default.frag")),
        ])?;

        shader.set_uniform("br", UniformValue::F32(0.5));

        let verts = [
            // Positions        //Colors
            -0.5,  0.5, 0.0,    1.0, 0.0, 0.0,
            -0.5, -0.5, 0.0,    1.0, 1.0, 0.0,
             0.5, -0.5, 0.0,    1.0, 0.0, 1.0,
             0.5,  0.5, 0.0,    0.0, 1.0, 1.0,
        ];

        let indices = [0, 1, 3, 3, 1, 2];

        let vbo = Vbo::new(&verts, glow::STATIC_DRAW, VboFormat::PositionColor(3, 3))?;
        renderer.load_vbo(0, vbo);

        let ebo = Ebo::new(&indices, glow::STATIC_DRAW)?;
        renderer.load_ebo(0, ebo);

        let vao = Vao::new(0, 0)?;
        renderer.load_vao(0, vao);

        renderer.load_shader("default", shader);

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
        let renderer = get_v39().renderer();

        match key
        {
            input::V39Key::Q => get_v39().quit(),
            input::V39Key::Up => {renderer.set_shader_uniform("default", "br", UniformValue::F32(1.0));},
            input::V39Key::Down => {renderer.set_shader_uniform("default", "br", UniformValue::F32(0.5));},
            _ => {},
        }
        Ok(())
    }

    // Each tick, render to the screen
    fn tick(&mut self, _delta: f32) -> V39Result<()> 
    {
        let renderer = get_v39().renderer();

        renderer.use_shader("default");
        let count = renderer.use_vao(0).unwrap_or(0);

        renderer.exec_gl(|gl| unsafe {
            gl.draw_elements(glow::TRIANGLES, count, glow::UNSIGNED_INT, 0);
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

