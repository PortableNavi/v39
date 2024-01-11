use crate::input::V39Key;
use winit::keyboard::KeyCode;


pub fn winit_key_to_v39_key(key: &KeyCode) -> V39Key
{
    //TODO: Complete this...

    match key
    {
        KeyCode::KeyQ => V39Key::Q,
        _ => V39Key::Space,
    }
}
