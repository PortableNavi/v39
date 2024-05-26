use v39::prelude::*;
use v39::math::*;


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

        let shader_id = renderer.load_shader(shader);

        let texture = Texture::from_file("gl-test/textures/miku.png")?;
        texture.set_params(&[
            (glow::TEXTURE_MIN_FILTER, TexParam::U32(glow::NEAREST)),
            (glow::TEXTURE_MAG_FILTER, TexParam::U32(glow::NEAREST)),
            (glow::TEXTURE_WRAP_S, TexParam::U32(glow::REPEAT)),
            (glow::TEXTURE_WRAP_T, TexParam::U32(glow::REPEAT)),
        ]);

        let texture_id = renderer.load_texture(texture);

        let verts = vec![
            // Positions        //Colors        //Coords
            -0.5, 0.0,  0.5,    1.0, 0.0, 0.0,  0.0, 0.0,
            -0.5, 0.0, -0.5,    1.0, 1.0, 0.0,  2.5, 0.0,
             0.5, 0.0, -0.5,    1.0, 0.0, 1.0,  0.5, 0.0,
             0.5, 0.0,  0.5,    0.0, 1.0, 1.0,  2.5, 0.5,
             0.0, 0.8,  0.0,    0.0, 1.0, 1.0,  1.25, 2.5,
        ];

        let indices = vec![
            0, 1, 2,
            0, 2, 3,
            0, 1, 4,
            1, 2, 4,
            2, 3, 4,
            3, 0, 4,
        ];

        let model = Model::new(
            verts,
            VboFormat::PositionColorCoords(3, 3, 2),
            indices,
            glow::STATIC_DRAW,
            shader_id,
            &[(texture_id, glow::TEXTURE0, "tex")],
        )?;

        renderer.load_model(model);
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
    fn tick(&mut self, delta: f32) -> V39Result<()> 
    {
        let renderer = get_v39().renderer();

        if let Some(model) = renderer.get_model(ModelId(0))
        {
            model.transform(|mat| {glm::rotate_y(mat, 0.7*delta)})
        }

        let count = renderer.use_model(ModelId(0)).unwrap_or(0);

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

