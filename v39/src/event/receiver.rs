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
}
