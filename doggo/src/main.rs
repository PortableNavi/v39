use v39::prelude::*;
use v39::math::*;


#[derive(Default)]
struct App
{
    model: ModelId,
    cam: Camera,
}


impl EventReceiver for App
{
    // Init stuff
    fn reset(&mut self) -> V39Result<()> 
    {
        get_v39().timer().set_target_fps(Some(60));

        if let Some(id) = model_setup()?
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
        {
            let cam_speed = 1.0;
            let i = get_v39().input_manager();
            let mut translation = glm::Vec3::new(0.0, 0.0, 0.0);
            let mut rotation = glm::Vec3::new(0.0, 0.0, 0.0);

            if i.is_held(input::V39Key::W) {translation.z += cam_speed * delta}
            if i.is_held(input::V39Key::S) {translation.z -= cam_speed * delta}
            if i.is_held(input::V39Key::A) {translation.x += cam_speed * delta}
            if i.is_held(input::V39Key::D) {translation.x -= cam_speed * delta}
            if i.is_held(input::V39Key::Shift) {translation.y += cam_speed * delta}
            if i.is_held(input::V39Key::Space) {translation.y -= cam_speed * delta}
            if i.is_held(input::V39Key::Q) {rotation.z += cam_speed * delta};
            if i.is_held(input::V39Key::E) {rotation.z -= cam_speed * delta};

            self.cam.transform_view(|v|{v.append_translation(&translation)});
        }

        let renderer = get_v39().renderer();

        if let Some(model) = renderer.get_model(self.model)
        {
            model.transform(|mat| {glm::rotate_y(mat, 0.7*delta)});
        }

        renderer.draw_model(self.model, &self.cam);

        Ok(())
    }
}


fn main() -> V39Result<()>
{
    let props = InitProps {title: "Doggo Verse".into(), ..InitProps::default()};
    let app = v39::init(&props)?;
    app.event_handler().add_receiver(App::default());
    app.run()
}


fn model_setup() -> V39Result<Option<ModelId>>
{
    let renderer = get_v39().renderer();

    let shader = Shader::new(&[
        ShaderSource::vertex(include_str!("../shaders/default.vert")),
        ShaderSource::fragment(include_str!("../shaders/default.frag")),
    ])?;

    let shader_id = renderer.load_shader(shader);

    let texture = Texture::from_bytes(include_bytes!("../textures/doggo.png"))?;
    texture.set_params(&[
        (glow::TEXTURE_MIN_FILTER, TexParam::U32(glow::NEAREST)),
        (glow::TEXTURE_MAG_FILTER, TexParam::U32(glow::NEAREST)),
        (glow::TEXTURE_WRAP_S, TexParam::U32(glow::REPEAT)),
        (glow::TEXTURE_WRAP_T, TexParam::U32(glow::REPEAT)),
    ]);

    let texture_id = renderer.load_texture(texture);
    let scale = 0.5;

    let verts = vec![
        // Positions        //Colors        //Coords
        -0.5, 0.0,  0.5,    1.0, 0.0, 0.0,  0.0, 0.0,
        -0.5, 0.0, -0.5,    1.0, 1.0, 0.0,  2.5/scale, 0.0,
         0.5, 0.0, -0.5,    1.0, 0.0, 1.0,  0.5/scale, 0.0,
         0.5, 0.0,  0.5,    0.0, 1.0, 1.0,  2.5/scale, 0.5/scale,
         0.0, 0.8,  0.0,    0.0, 1.0, 1.0,  1.25/scale, 2.5/scale,
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

    Ok(renderer.load_model(model))
}

