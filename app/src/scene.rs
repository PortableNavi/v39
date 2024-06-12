use v39::prelude::*;


/// Small functions to define and load the shaders, textures and models.


pub fn load_shaders() -> V39Result<ShaderId>
{
    let renderer = get_v39().renderer();

    let shader = Shader::new(&[
        ShaderSource::vertex(include_str!("../shaders/default.vert")),
        ShaderSource::fragment(include_str!("../shaders/default.frag")),
    ])?;

    let shader_id = renderer.load_shader(shader);

    Ok(shader_id)
}


pub fn load_textures() -> V39Result<TextureId>
{
    let renderer = get_v39().renderer();
    
    let texture = Texture::from_bytes(include_bytes!("../textures/miku.png"))?;
    texture.set_params(&[
        (glow::TEXTURE_MIN_FILTER, TexParam::U32(glow::NEAREST)),
        (glow::TEXTURE_MAG_FILTER, TexParam::U32(glow::NEAREST)),
        (glow::TEXTURE_WRAP_S, TexParam::U32(glow::REPEAT)),
        (glow::TEXTURE_WRAP_T, TexParam::U32(glow::REPEAT)),
    ]);

    let texture_id = renderer.load_texture(texture);

    Ok(texture_id)
}


pub fn load_model(shader: ShaderId, texture: TextureId) -> V39Result<Option<ModelId>>
{
    let renderer = get_v39().renderer();
    
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
        shader,
        &[(texture, glow::TEXTURE0, "tex")],
    )?;

    Ok(renderer.load_model(model))
}

