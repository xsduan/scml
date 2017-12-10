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

pub fn read(scml_str: &str) -> Scml {
    from_str(scml_str).unwrap()
}
