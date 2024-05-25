use crate::prelude::*;


pub struct Vao
{
    buffer: glow::VertexArray,
    vbo: usize,
    ebo: usize,
    count: u32,
}


impl Vao
{
    pub fn new(vbo: usize, ebo: usize) -> V39Result<Self>
    {
        if !get_v39().renderer().use_vbo(vbo)
        {
            return Err(V39Error::Renderer(format!("VBO({vbo}) is not loaded")));
        }

        if !get_v39().renderer().is_ebo_loaded(ebo)
        {
            return Err(V39Error::Renderer(format!("EBO({ebo}) is not loaded")));
        }

        let mut vao = None;
        let vbo_obj = get_v39().renderer().get_vbo(vbo).unwrap();
        let ebo_obj = get_v39().renderer().get_ebo(ebo).unwrap();

        get_v39().renderer().exec_gl(|gl| unsafe {
            vao = Some(gl.create_vertex_array()?);
            gl.bind_vertex_array(vao);
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo_obj.buffer()));

            let format = vbo_obj.format();

            match format
            {
                VboFormat::Position(psize) => {
                    gl.enable_vertex_attrib_array(0);
                    gl.vertex_attrib_pointer_f32(0, psize, vbo_obj.kind(), false, 0, 0);
                },

                VboFormat::PositionColor(psize, csize) => {
                    gl.enable_vertex_attrib_array(0);
                    gl.vertex_attrib_pointer_f32(0, psize, vbo_obj.kind(), false, 8*csize, 0);
                    gl.enable_vertex_attrib_array(1);
                    gl.vertex_attrib_pointer_f32(1, csize, vbo_obj.kind(), false, 8*psize, 4*psize);
                },
            }

            gl.bind_vertex_array(None);
            Ok(())
        })?;

        get_v39().renderer().clear_vbo();
        get_v39().renderer().clear_ebo();

        match vao
        {
            Some(buffer) => Ok(Self {buffer, vbo, ebo, count: ebo_obj.count()}),
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

    pub(crate) fn ebo(&self) -> usize
    {
        self.ebo
    }

    pub(crate) fn count(&self) -> u32
    {
        self.count
    }
}

