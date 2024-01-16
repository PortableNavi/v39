use std::time::{Instant, Duration};
use std::sync::Mutex;
use once_cell::sync::OnceCell;
use crate::prelude::*;


static INSTANCE: OnceCell<Timer> = OnceCell::new();


pub(crate) struct Timer
{
    frame_tracker: Mutex<Tracker>,
    delta: Mutex<Duration>,
    target_frame_time: Mutex<Option<Duration>>,
}


impl Timer
{
    pub(crate) fn init() -> V39Result<&'static Self>
    {
        let timer = Timer {
            frame_tracker: Mutex::new(Tracker::new()),
            delta: Mutex::new(Duration::from_secs(0)),
            target_frame_time: Mutex::new(None),
        };

        if INSTANCE.set(timer).is_err()
        {
            return Err(V39Error::Reinit("Timer".into()));
        }

        Ok(INSTANCE.get().unwrap())
    }

    pub(crate) fn start_frame_tracker(&self)
    {
        if let Ok(mut tracker) = self.frame_tracker.lock()
        {
            *tracker = Tracker::new();
        }
    }

    pub(crate) fn end_frame_tracker(&self)
    {
        if let (Ok(mut tracker), Ok(mut delta)) = (self.frame_tracker.lock(), self.delta.lock())
        {
            *delta = tracker.stop();
        }
    }

    pub(crate) fn delta_time(&self) -> Duration
    {
        *self.delta.lock().expect("Failure isn't an option")
    }

    pub(crate) fn current_frame_time(&self) -> Duration
    {
        self.frame_tracker.lock().unwrap().peek()
    }

    pub(crate) fn frame_time(&self) -> Option<Duration>
    {
        *self.target_frame_time.lock().unwrap()
    }

    pub(crate) fn set_target_frame_time(&self, target: Option<Duration>)
    {
        if let Ok(mut ft) = self.target_frame_time.lock()
        {
            *ft = target;
        }
    }
}


#[derive(Copy, Clone, Debug)]
pub struct Tracker
{
    begin: Instant,
    end: Option<Instant>,
}


impl Tracker
{
    fn new() -> Self
    {
        Self {
            begin: Instant::now(),
            end: None,
        }
    }

    fn stop(&mut self) -> Duration
    {
        if self.is_tracking()
        {
            self.end = Some(Instant::now());
        }

        let end = self.end.unwrap_or_else(Instant::now);
        end - self.begin
    }

    fn is_tracking(&self) -> bool
    {
        self.end.is_none()
    }

    fn peek(&self) -> Duration
    {
        Instant::now() - self.begin
    }
}
