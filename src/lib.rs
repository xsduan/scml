#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_json;

pub mod parse;

pub use parse::{parse};

/// Implementation

use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

use parse::Scml;

pub fn read(filename: &str) -> Result<Scml, Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(parse(&contents))
}
