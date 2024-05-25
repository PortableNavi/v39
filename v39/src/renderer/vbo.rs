use crate::prelude::*;
use std::marker::PhantomData;
use crate::renderer::to_bytes;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VboFormat
{
    Position(i32),
    PositionColor(i32, i32),
    PositionCoords(i32, i32),
    PositionColorCoords(i32, i32, i32)
}


#[derive(Clone)]
pub struct Vbo<const T: u32, D>
{
    buffer: glow::Buffer,
    format: VboFormat,
    _phantom: PhantomData<D>,
}


impl<const T: u32, D> Vbo<T, D>
{
    pub fn new(data: &[D], draw_mode: u32, format: VboFormat) -> V39Result<Self>
    {
        let mut vbo = None;

        get_v39().renderer().exec_gl(|gl| unsafe {
            vbo = Some(gl.create_buffer()?);
            gl.bind_buffer(glow::ARRAY_BUFFER, vbo);
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, to_bytes(data), draw_mode);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);

            Ok(())
        });

        match vbo
        {
            Some(buffer) => Ok(Self {buffer, format, _phantom: PhantomData}),
            None => Err(V39Error::Renderer("Failed to create vbo".into())) //UNREACHABLE
        }
    }

    pub fn kind(&self) -> u32
    {
        T
    }

    pub fn format(&self) -> VboFormat
    {
        self.format
    }

    pub(crate) fn buffer(&self) -> glow::Buffer
    {
        self.buffer
    }
}


impl<const T: u32, D> Drop for Vbo<T, D>
{
    fn drop(&mut self) 
    {
        let _ = get_v39().renderer().exec_gl(|gl| unsafe {
            gl.delete_buffer(self.buffer);
            Ok(())
        });
    }
}
