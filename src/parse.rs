use serde_xml_rs::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Scml {
    #[serde(rename = "stroke")]
    pub strokes: Vec<Stroke>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stroke {
    #[serde(rename = "type")]
    pub stroke_type: String,
    #[serde(rename = "anchor", default)]
    pub anchors: Vec<Anchor>,
    #[serde(rename = "location", default)]
    pub locations: Vec<Location>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Anchor {
    pub id: String,
    pub at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub id: String,
    pub at: String,
}

pub fn read(scml_raw: &str) -> Scml {
    deserialize(scml_raw.as_bytes()).unwrap()
}
