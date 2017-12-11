extern crate scml;

use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

use scml::parse::Parse;

fn main() {
    let scml_json = read("src/bin/examples/scml_65e5.json").unwrap();
    let stroke_json = read("src/bin/examples/cjk_strokes.json").unwrap();

    // println!("{:#?}", scml::Scml::parse(&scml_json));
    println!("{:#?}", scml::StrokeDictionary::parse(&stroke_json));
}

fn read(filename: &str) -> Result<String, Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}
