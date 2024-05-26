use std::sync::atomic::{AtomicU32, Ordering};


static MODEL_ID: AtomicU32 = AtomicU32::new(0);
static SHADER_ID: AtomicU32 = AtomicU32::new(0);
static TEXTURE_ID: AtomicU32 = AtomicU32::new(0);


#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub struct ModelId(pub u32);
impl ModelId
{
    pub fn new() -> Self
    {
        Self(MODEL_ID.fetch_add(1, Ordering::SeqCst))
    }
}


#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub struct ShaderId(pub u32);
impl ShaderId
{
    pub fn new() -> Self
    {
        Self(SHADER_ID.fetch_add(1, Ordering::SeqCst))
    }
}


#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub struct TextureId(pub u32);
impl TextureId
{
    pub fn new() -> Self
    {
        Self(TEXTURE_ID.fetch_add(1, Ordering::SeqCst))
    }
}

