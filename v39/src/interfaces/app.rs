use once_cell::sync::OnceCell;
use crate::prelude::*;


static INSTANCE: OnceCell<App> = OnceCell::new();


pub struct App
{
    
}


impl App
{
    pub(crate) fn init() -> V39Result<()>
    {
        let app = App {};

        if INSTANCE.set(app).is_err()
        {
            return Err(V39Error::Reinit("App".into()));
        }

        Ok(())
    }

    pub fn get() -> &'static App
    {
        INSTANCE.get().expect("App instance was not initialized")
    }
}
