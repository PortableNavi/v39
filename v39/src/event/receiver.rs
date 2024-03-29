use crate::prelude::*;


pub trait EventReceiver
{
    fn dispatch_event(&mut self, event: Event) -> V39Result<()>
    {
        Ok(())
    }

    fn reset(&mut self) -> V39Result<()>
    {
        Ok(())
    }

    fn key_down(&mut self, key: input::V39Key) -> V39Result<()>
    {
        Ok(())
    }

    fn key_up(&mut self, key: input::V39Key) -> V39Result<()>
    {
        Ok(())
    }

    fn frame_end(&mut self) -> V39Result<()>
    {
        Ok(())
    }
    
    fn frame_begin(&mut self) -> V39Result<()>
    {
        Ok(())
    }
    
    fn tick(&mut self, delta: f32) -> V39Result<()>
    {
        Ok(())
    }

    fn quit(&mut self, reason: u32) -> V39Result<()>
    {
        Ok(())
    }

    fn window_close(&mut self) -> V39Result<()>
    {
        Ok(())
    }

    fn window_resize(&mut self, size: (u32, u32)) -> V39Result<()>
    {
        Ok(())
    }

    fn window_focus(&mut self) -> V39Result<()>
    {
        Ok(())
    }

    fn window_unfocus(&mut self) -> V39Result<()>
    {
        Ok(())
    }

}
