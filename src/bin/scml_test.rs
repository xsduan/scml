extern crate scml;

use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

use scml::*;

fn main() {
    let scml_json = Scml::parse(&read("src/bin/examples/scml_65e5.json").unwrap());
    let stroke_json = StrokeDictionary::parse(&read("src/bin/examples/cjk_strokes.json").unwrap());

    scml::transform(&scml_json, &stroke_json);
}

fn read(filename: &str) -> Result<String, Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}
