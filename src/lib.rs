#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate svg;

pub mod parse;
pub mod process;
pub mod point;

pub use parse::{Scml, StrokeDictionary, Parse};
pub use process::transform;
