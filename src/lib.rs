#![feature(proc_macro, generators)]
extern crate futures_await as futures;
extern crate futures_retry;
extern crate regex;
extern crate telegram_bot;
extern crate tokio_core;

mod app;

pub use self::app::{App, AppError, AppResult};
