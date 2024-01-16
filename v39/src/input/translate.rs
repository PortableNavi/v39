use crate::input::V39Key;
use winit::keyboard::KeyCode;


pub fn winit_key_to_v39_key(key: &KeyCode) -> V39Key
{
    //TODO: Complete this...

    match key
    {
        KeyCode::KeyA => V39Key::A,
        KeyCode::KeyQ => V39Key::Q,
        KeyCode::KeyF => V39Key::F,
        _ => V39Key::Space,
    }
}
