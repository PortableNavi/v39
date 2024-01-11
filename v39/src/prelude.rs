use crate::interfaces::app::App;


pub use crate::error::V39Error;
pub use crate::event::receiver::EventReceiver;
pub use crate::event::event::{Event, EventData};
pub use crate::input;


pub type V39Result<T> = Result<T, V39Error>;


#[inline]
pub fn get_v39() -> &'static App
{
    App::get()
}
