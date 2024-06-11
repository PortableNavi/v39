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
        KeyCode::KeyW => V39Key::W,
        KeyCode::KeyS => V39Key::S,
        KeyCode::KeyD => V39Key::D,
        KeyCode::ArrowUp => V39Key::Up,
        KeyCode::ArrowDown => V39Key::Down,
        KeyCode::ArrowLeft => V39Key::Left,
        KeyCode::ArrowRight => V39Key::Right,
        KeyCode::Space => V39Key::Space,
        KeyCode::ShiftLeft => V39Key::Shift,
        _ => V39Key::F10,
    }
}
