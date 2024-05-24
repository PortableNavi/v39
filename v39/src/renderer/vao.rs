use crate::prelude::*;


pub struct Vao
{
    buffer: glow::VertexArray,
    vbo: usize,
}


impl Vao
{
    pub fn new(vbo: usize) -> V39Result<Self>
    {
        if !get_v39().renderer().use_vbo(vbo)
        {
            return Err(V39Error::Renderer(format!("VBO({vbo}) is not loaded")));
        }

        let mut vao = None;
        let vbo_obj = get_v39().renderer().get_vbo(vbo).unwrap();

        get_v39().renderer().exec_gl(|gl| unsafe {
            vao = Some(gl.create_vertex_array()?);
            gl.bind_vertex_array(vao);
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, vbo_obj.size(), vbo_obj.kind(), false, 0, 0);
            gl.bind_vertex_array(None);
            Ok(())
        })?;

        get_v39().renderer().clear_vbo();

        match vao
        {
            Some(buffer) => Ok(Self {buffer, vbo}),
            None => Err(V39Error::Renderer("Failed to create vao".into())) //UNREACHABLE
        }
    }

    pub(crate) fn buffer(&self) -> glow::VertexArray
    {
        self.buffer
    }

    pub(crate) fn vbo(&self) -> usize
    {
        self.vbo
    }
}

