use crate::prelude::*;
use crate::renderer::to_bytes;


#[derive(Clone)]
pub struct Ebo
{
    buffer: glow::Buffer,
    count: u32,
}


impl Ebo
{
    pub fn new(indices: &[u32], draw_mode: u32) -> V39Result<Self>
    {
        let mut buffer = None;

        get_v39().renderer().exec_gl(|gl| unsafe {
            buffer = Some(gl.create_buffer()?);
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, buffer);
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, to_bytes(indices), draw_mode);
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
            Ok(())
        })?;

        match buffer
        {
            Some(buffer) => Ok(Self {buffer, count: indices.len() as u32}),
            None => Err(V39Error::Renderer("Failed to create ebo".into())) //UNREACHABLE
        }
    }

    pub(crate) fn buffer(&self) -> glow::Buffer
    {
        self.buffer
    }

    pub(crate) fn count(&self) -> u32
    {
        self.count
    }
}


impl Drop for Ebo
{
    fn drop(&mut self) 
    {
        let _ = get_v39().renderer().exec_gl(|gl| unsafe {
            gl.delete_buffer(self.buffer);
            Ok(())
        });
    }
}
