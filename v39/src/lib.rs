#![allow(clippy::suspicious_else_formatting)]
#![allow(unused)]


pub mod error;
pub mod prelude;
pub mod interfaces;
pub mod event;
pub mod input;

use prelude::*;
use interfaces::app::App;

use pretty_env_logger::{env_logger::Env, env_logger::fmt::Formatter};
use log::Record;
use std::io::Write;
use std::env;

#[macro_use]
extern crate log;


pub fn init() -> V39Result<&'static App>
{
    init_logger()?;
    info!("Logger initialized");

    App::init()?;
    info!("App Interface initialized");

    Ok(App::get())
}


fn init_logger() -> V39Result<()>
{
    if env::var("V39_DISCARD_LOG").is_ok()
    {
        return Ok(());
    }

    let fallback_level = {
        if cfg!(debug_assertions) {"trace"}
        else {"error"}
    };

    let format = {
        if cfg!(debug_assertions)
        {
            |buf: &mut Formatter, record: &Record| {
                
                writeln!(buf, "[{}({}) - {}:{}]: {}",
                    record.level(),
                    chrono::Local::now().format("%H:%M:%S%.3f"),
                    record.file().unwrap_or(""),
                    record.line().unwrap_or(0),
                    record.args())
            }
        }

        else
        {
             |buf: &mut Formatter, record: &Record| {
                
                writeln!(buf, "[{} - {}]: {}",
                    record.level(),
                    chrono::Local::now().format("%H:%M:%S%.3f"),
                    record.args())
            }
        }
    };

    let env = Env::default()
        .default_filter_or(fallback_level);

    pretty_env_logger::formatted_builder()
        .parse_env(env)
        .format(format)
        .init();

    Ok(())
}
