use std::sync::{Arc, Mutex};
use v39::prelude::*;
use v39::math::glm;


#[derive(Default)]
pub struct MainCamera
{
    cam: Arc<Mutex<Camera>>,
    speed: f32
}


impl EventReceiver for MainCamera
{
    fn reset(&mut self) -> V39Result<()> 
    {
        self.speed = 1.0;
        get_v39().renderer().set_camera(self.cam.clone());
        Ok(())
    }

    fn tick(&mut self, delta: f32) -> V39Result<()> 
    {
        let i = get_v39().input_manager();
        let mut translation = glm::Vec3::new(0.0, 0.0, 0.0);
        let mut rotation = glm::Vec3::new(0.0, 0.0, 0.0);

        if i.is_held(input::V39Key::W)      {translation.z += self.speed * delta}
        if i.is_held(input::V39Key::S)      {translation.z -= self.speed * delta}
        if i.is_held(input::V39Key::A)      {translation.x += self.speed * delta}
        if i.is_held(input::V39Key::D)      {translation.x -= self.speed * delta}
        if i.is_held(input::V39Key::Shift)  {translation.y += self.speed * delta}
        if i.is_held(input::V39Key::Space)  {translation.y -= self.speed * delta}
        if i.is_held(input::V39Key::Q)      {rotation.z += self.speed * delta};
        if i.is_held(input::V39Key::E)      {rotation.z -= self.speed * delta};

        self.cam.lock()
            .unwrap()
            .transform_view(|v|{v.append_translation(&translation)});
        
        Ok(())
    }

    fn window_resize(&mut self, size: (u32, u32)) -> V39Result<()> 
    {
        self.cam.lock()
            .unwrap()
            .set_aspect(size.0 as f32 / size.1 as f32);

        Ok(())
    }
}

