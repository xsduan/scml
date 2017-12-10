use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

use serde_json::from_str;

#[derive(Deserialize, Debug)]
pub struct Place {
    id: u16,
    at: String,
}

#[derive(Deserialize, Debug)]
pub struct Stroke {
    anchors: Vec<Place>,
    locations: Vec<Place>,
    #[serde(rename = "type")]
    pub stroke_type: String,
}

#[derive(Deserialize, Debug)]
pub struct Scml {
    pub strokes: Vec<Stroke>,
}

pub fn parse(scml_str: &str) -> Scml {
    from_str(scml_str).expect("Scml invalid")
}

pub fn read(filename: &str) -> Result<Scml, Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(parse(&contents))
}
