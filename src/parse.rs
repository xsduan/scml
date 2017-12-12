use std::collections::HashMap;

use serde_json::from_str;

pub trait Parse {
    fn parse(data: &str) -> Self;
}

#[derive(Deserialize, Debug)]
pub struct Place {
    pub id: usize,
    pub at: String,
}

#[derive(Deserialize, Debug)]
pub struct Stroke {
    pub anchors: Vec<Place>,
    pub locations: Vec<Place>,
    #[serde(rename = "type")]
    pub stroke_type: String,
}

#[derive(Deserialize, Debug)]
pub struct Scml {
    pub strokes: Vec<Stroke>,
}

impl Parse for Scml {
    fn parse(scml_json: &str) -> Scml {
        from_str(scml_json).expect("Scml parse error")
    }
}

#[derive(Deserialize, Debug, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Anchor {
    pub name: String,
    pub point: Point,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Location {
    pub name: String,
    pub first: String,
    pub second: String,
    pub direction: Point,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StrokeDescription {
    pub anchors: Vec<Anchor>,
    pub locations: Vec<Location>,
}

#[derive(Deserialize, Debug)]
pub struct StrokeDictionary {
    pub tag: String,
    pub descriptions: HashMap<String, StrokeDescription>,
}

impl Parse for StrokeDictionary {
    fn parse(stroke_json: &str) -> StrokeDictionary {
        from_str(stroke_json).expect("Stroke description parse error")
    }
}
