#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_json;

pub mod parse;

pub use parse::{parse, read};
