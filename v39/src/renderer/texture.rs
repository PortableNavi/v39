use crate::prelude::*;


#[derive(Clone, PartialEq, Debug)]
pub enum TexParam<'a>
{
    U32(u32),
    U32Slice(&'a[u32]),
    F32(f32),
    F32Slice(&'a[f32]),
}


#[derive(Clone, PartialEq, Debug)]
pub struct Texture
{
    image: glow::Texture,
    width: u32,
    height: u32,
}


impl Texture
{
    pub fn new(data: &[u8], width: u32, height: u32) -> V39Result<Self>
    {
        let mut image = None;

        get_v39().renderer().exec_gl(|gl| unsafe {
            image = Some(gl.create_texture()?);
            gl.bind_texture(glow::TEXTURE_2D, image);
            
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                width as i32,
                height as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                Some(data),
            );
            
            gl.generate_mipmap(glow::TEXTURE_2D);

            gl.bind_texture(glow::TEXTURE_2D, None);
            Ok(())
        })?;

        match image
        {
            Some(image) => Ok(Self {image, width, height}),
            None => Err(V39Error::Renderer("Failed to create GL Texture".into())),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> V39Result<Self>
    {
        let data = image::load_from_memory(bytes)?.into_rgba8();
        Self::new(&data, data.width(), data.height())
    }

    pub fn from_file(path: impl AsRef<std::path::Path>) -> V39Result<Self>
    {
        let data = image::open(path)?.into_rgba8();
        Self::new(&data, data.width(), data.height())
    }

    pub fn set_params(&self, parameters: &[(u32, TexParam)])
    {
        let _ = get_v39().renderer().exec_gl(|gl| unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(self.image));

            for (param, val) in parameters
            {
                match val
                {
                    TexParam::U32(val) => gl.tex_parameter_i32(glow::TEXTURE_2D, *param, *val as i32),
                    TexParam::F32(val) => gl.tex_parameter_f32(glow::TEXTURE_2D, *param, *val),
                    TexParam::U32Slice(val) => gl.tex_parameter_i32_slice(glow::TEXTURE_2D, *param, &val.iter().map(|v| *v as i32).collect::<Vec<_>>()),
                    TexParam::F32Slice(val) => gl.tex_parameter_f32_slice(glow::TEXTURE_2D, *param, val),
                }
            }
            
            gl.bind_texture(glow::TEXTURE_2D, None);
            Ok(())
        });
    }

    pub(crate) fn image(&self) -> glow::Texture
    {
        self.image
    }

    pub fn width(&self) -> u32
    {
        self.width
    }

    pub fn height(&self) -> u32
    {
        self.height
    }

    pub fn assign(&self, unit: u32)
    {
        let _ = get_v39().renderer().exec_gl(|gl| unsafe {
            gl.active_texture(unit);
            Ok(())
        });
    }
}


impl Drop for Texture
{
    fn drop(&mut self) 
    {
        let _ = get_v39().renderer().exec_gl(|gl| unsafe {
            gl.delete_texture(self.image);
            Ok(())
        });
    }
}

