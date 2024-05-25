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

        let verts = [
            // Positions        //Colors        //Coords
            -0.5,  0.5, 0.0,    1.0, 0.0, 0.0,  0.0, 1.0,
            -0.5, -0.5, 0.0,    1.0, 1.0, 0.0,  0.0, 0.0,
             0.5, -0.5, 0.0,    1.0, 0.0, 1.0,  1.0, 0.0,
             0.5,  0.5, 0.0,    0.0, 1.0, 1.0,  1.0, 1.0,
        ];

        let indices = [0, 1, 3, 3, 1, 2];

        let vbo = Vbo::new(&verts, glow::STATIC_DRAW, VboFormat::PositionColorCoords(3, 3, 2))?;
        renderer.load_vbo(0, vbo);

        let ebo = Ebo::new(&indices, glow::STATIC_DRAW)?;
        renderer.load_ebo(0, ebo);

        let vao = Vao::new(0, 0)?;
        renderer.load_vao(0, vao);

        let texture = Texture::from_file("gl-test/textures/miku.png")?;
        texture.set_params(&[
            (glow::TEXTURE_MIN_FILTER, TexParam::U32(glow::NEAREST)),
            (glow::TEXTURE_MAG_FILTER, TexParam::U32(glow::NEAREST)),
            (glow::TEXTURE_WRAP_S, TexParam::U32(glow::CLAMP_TO_BORDER)),
            (glow::TEXTURE_WRAP_T, TexParam::U32(glow::CLAMP_TO_BORDER)),
            (glow::TEXTURE_BORDER_COLOR, TexParam::F32Slice(&[1.0, 1.0, 1.0, 1.0])),
        ]);

        renderer.load_texture(0, texture);
        
        renderer.load_shader(0, shader);
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
        if key == input::V39Key::Q {get_v39().quit()}
        Ok(())
    }

    // Each tick, render to the screen
    fn tick(&mut self, _delta: f32) -> V39Result<()> 
    {
        let renderer = get_v39().renderer();
        let count = renderer.use_vao(0).unwrap_or(0);
        renderer.use_texture(0, glow::TEXTURE0, 0, "tex");

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

