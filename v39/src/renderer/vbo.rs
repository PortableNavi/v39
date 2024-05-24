use crate::prelude::*;
use std::marker::PhantomData;


#[derive(Clone)]
pub struct Vbo<const T: u32, D>
{
    buffer: glow::Buffer,
    size: i32,
    _phantom: PhantomData<D>,
}


impl<const T: u32, D> Vbo<T, D>
{
    pub fn new(data: &[D], draw_mode: u32) -> V39Result<Self>
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
            Some(buffer) => Ok(Self {buffer, size: 3, _phantom: PhantomData}),
            None => Err(V39Error::Renderer("Failed to create vbo".into())) //UNREACHABLE
        }
    }

    pub fn with_size(data: &[D], draw_mode: u32, size: i32) -> V39Result<Self>
    {
        let mut me = Self::new(data, draw_mode)?;
        me.size = size;
        Ok(me)
    }

    pub fn kind(&self) -> u32
    {
        T
    }

    pub fn size(&self) -> i32
    {
        self.size
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


unsafe fn to_bytes<T>(slice: &[T]) -> &[u8]
{
    core::slice::from_raw_parts(
        slice.as_ptr() as *const u8,
        std::mem::size_of_val(slice),
    )
}

