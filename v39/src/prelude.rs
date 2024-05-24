use crate::interfaces::app::App;

pub use glow;
pub use glow::HasContext;
pub use crate::error::V39Error;
pub use crate::event::receiver::EventReceiver;
pub use crate::event::event::{Event, EventData};
pub use crate::input;
pub use crate::renderer::{Shader, ShaderSource, Vbo, Vao};
pub use crate::InitProps;

pub type V39Result<T> = Result<T, V39Error>;


#[inline]
pub fn get_v39() -> &'static App
{
    App::get()
}
