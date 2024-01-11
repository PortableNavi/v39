use crate::prelude::*;
use crate::input::InputManager;


#[derive(Clone)]
pub struct InputManagerInterface
{
    handler: &'static InputManager,
}


impl InputManagerInterface
{
    pub(crate) fn new() -> V39Result<Self>
    {
        let handler = InputManager::init()?;
        info!("Input Manager Initialized");

        Ok(Self {handler})
    }

    pub fn is_down(&self, key: input::V39Key) -> bool
    {
        self.handler.down_snapshot_contains(key)
    }

    pub fn is_held(&self, key: input::V39Key) -> bool
    {
        self.handler.held_snapshot_contains(key)
    }

    pub(crate) fn event_begin(&self)
    {
        self.handler.snapshot_keys_down();
        self.handler.snapshot_keys_up();
        self.handler.snapshot_keys_held();
    }

    pub(crate) fn event_end(&self)
    {
        self.handler.apply_down_snapshot();
        self.handler.apply_up_snapshot();
        self.handler.apply_held_snapshot();
    }
}

impl EventReceiver for InputManagerInterface
{
    fn frame_begin(&mut self) -> V39Result<()>
    {
        self.handler.down_held_up_conversion();
        Ok(())
    }

    fn frame_end(&mut self) -> V39Result<()> 
    {
        self.handler.clear_down_snapshot();
        Ok(())    
    }

    fn key_up(&mut self, key: input::V39Key) -> V39Result<()> 
    {
        self.handler.push_key_up(key);
        Ok(())
    }

    fn key_down(&mut self, key: input::V39Key) -> V39Result<()> 
    {
        self.handler.push_key_down(key);
        Ok(())
    }
}

