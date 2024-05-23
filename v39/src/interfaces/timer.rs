use crate::timer::Timer;
use crate::prelude::*;
use std::time::Duration;


#[derive(Clone)]
pub struct TimerInterface
{
    handle: &'static Timer
}


impl TimerInterface
{
    pub(crate) fn new() -> V39Result<Self>
    {
        let handle = Timer::init()?;
        Ok(Self {handle})
    }

    #[inline]
    pub fn delta_time(&self) -> Duration
    {
        self.handle.delta_time()
    }

    pub(crate) fn pad_frame_time(&self)
    {
        if let Some(target) = self.handle.frame_time()
        {
            let delta = self.handle.current_frame_time();

            if delta < target
            {
                std::thread::sleep(target - delta);
            }
        }
    }

    pub fn set_target_fps(&self, target: Option<u64>)
    {
        if let Some(fps) = target
        {
            let ft = (1000u64).div_floor(fps);
            self.handle.set_target_frame_time(Some(Duration::from_millis(ft)))
        }

        else
        {
            self.handle.set_target_frame_time(None);
        }
    }
}


impl EventReceiver for TimerInterface
{
    fn frame_begin(&mut self) -> V39Result<()>
    {
        self.handle.end_frame_tracker();
        self.handle.start_frame_tracker();
        Ok(())
    } 
}

