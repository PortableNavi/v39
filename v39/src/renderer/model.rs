use crate::math::*;
use crate::prelude::*;
use std::sync::{Arc, Mutex};


#[derive(Debug)]
pub struct Model
{
    id: ModelId,
    transform: Mutex<Mat4>,
    shader: ShaderId,
    textures: Vec<(TextureId, u32, String)>,
}


impl Model
{
    pub fn new(
        verts: Vec<f32>,
        format: VboFormat, 
        indices: Vec<u32>, 
        draw_mode: u32, 
        shader: ShaderId, 
        textures: &[(TextureId, u32, &str)]
    ) -> V39Result<Self>
    {
        let id = ModelId::new();
        let transform = Mutex::new(Mat4::identity());
        let renderer = get_v39().renderer();

        let vbo = Vbo::new(&verts, draw_mode, format)?;
        renderer.load_vbo(id, vbo);

        let ebo = Ebo::new(&indices, draw_mode)?;
        renderer.load_ebo(id, ebo);

        let vao = Vao::new(id, id)?;
        renderer.load_vao(id, vao);

        let textures = textures
            .iter()
            .map(|(id, sampler, name)| (*id, *sampler, name.to_string()))
            .collect::<Vec<_>>();

        Ok(Self {id, transform, shader, textures})
    }

    pub fn id(&self) -> ModelId
    {
        self.id
    }

    pub fn shader(&self) -> ShaderId
    {
        self.shader
    }

    pub fn textures(&self) -> &[(TextureId, u32, String)]
    {
        &self.textures
    }

    pub fn transform(&self, f: impl FnOnce(&Mat4) -> Mat4)
    {
        let mut mat = self.transform.lock().unwrap();
        *mat = f(&mat);
    }

    pub fn get_transform(&self) -> Mat4
    {
        *self.transform.lock().unwrap()
    }
}

