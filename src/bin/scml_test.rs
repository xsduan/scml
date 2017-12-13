extern crate scml;

use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

use scml::*;

fn main() {
    let examples_dir = "src/bin/examples/";
    let scml = Scml::parse(&read(&format!("{}scml_65e5.json", examples_dir)).unwrap());
    let stroke =
        StrokeDictionary::parse(&read(&format!("{}cjk_strokes.json", examples_dir)).unwrap());

    write(
        &format!("{}/{}.svg", examples_dir, scml.name),
        &scml.transform(&stroke).unwrap(),
    ).expect("Could not save file");
}

fn read(filename: &str) -> Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn write(filename: &str, data: &str) -> Result<usize> {
    let mut file = File::create(filename)?;

    file.write(data.as_bytes())
}
