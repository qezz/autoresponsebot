#![feature(box_patterns)]
extern crate rand;
extern crate ratelimit_meter;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate teleborg;

mod app;
mod rules;

pub use self::app::run;
pub use self::rules::load_from_file as load_rules;
