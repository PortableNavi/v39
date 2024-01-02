use crate::prelude::*;


pub trait EventReceiver
{
    fn dispatch_event(&mut self, event: Event, handler: &mut EventHandlerInterface) -> V39Result<()>
    {
        Ok(())
    }

    fn reset(&mut self, handler: &mut EventHandlerInterface) -> V39Result<()>
    {
        Ok(())
    }
}
