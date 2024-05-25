use thiserror::Error;


#[derive(Error, Debug)]
pub enum V39Error
{
    #[error("Reinitialization of already initialized item {0} is invalid")]
    Reinit(String),

    #[error("{0}")]
    NoSuitableDevie(String),

    #[error("{0}")]
    GlError(String),

    #[error("{0}")]
    Renderer(String),

    #[error("{0}")]
    Image(#[from] image::error::ImageError),

    #[error("{0}")]
    Error(String),
}


impl From<raw_gl_context::GlError> for V39Error
{
    fn from(value: raw_gl_context::GlError) -> Self 
    {
        Self::GlError(format!("{value:?}"))
    }
}


impl From<String> for V39Error
{
    fn from(value: String) -> Self
    {
        Self::Error(value)
    }
}
