extern crate glob;

extern crate scml;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::Result;

use glob::*;

use scml::*;

fn main() {
    let examples_dir = "src/bin/examples/";
    let scml_dir = format!("{}scml/", examples_dir);
    let output_dir = format!("{}svg/", examples_dir);

    let stroke = StrokeDictionary::parse(&read(
        Path::new(&format!("{}cjk_strokes.json", examples_dir)),
    ).unwrap());

    for entry in glob(&format!("{}*.json", scml_dir)).expect("Glob didn't work :(") {
        let scml = match entry {
            Ok(path) => Scml::parse(&read(path.as_path()).expect("Read error")),
            Err(e) => {
                println!("{:?}", e);
                continue;
            }
        };

        write(
            &format!("{}{}.svg", output_dir, scml.name),
            &scml.transform(&stroke).unwrap(),
        ).expect("Could not save file");
    }
}

fn read(filename: &Path) -> Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn write(filename: &str, data: &str) -> Result<usize> {
    let mut file = File::create(filename)?;

    file.write(data.as_bytes())
}
