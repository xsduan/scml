use std::f32;
use std::str;
use std::io::Result;

use svg::*;
use svg::node::element::Path;
use svg::node::element::path::Data;

use parse::*;
use point::*;

impl Scml {
    pub fn stroke_count(&self) -> usize {
        self.strokes.len()
    }

    pub fn transform(&self, strokes: &StrokeDictionary) -> Result<String> {
        let mut stroke_points = Vec::with_capacity(self.stroke_count());
        let mut places = Vec::new();

        for stroke in self.strokes.iter() {
            // store copy of default, normalized stroke
            let mut this_stroke;
            match strokes.get(&stroke.stroke_type) {
                Some(thing) => this_stroke = thing.clone(),
                None => {
                    panic!(
                        "Invalid stroke type {} for stroke set {}",
                        stroke.stroke_type,
                        strokes.tag
                    )
                }
            }

            for anchor_place in stroke.anchors.iter() {
                if anchor_place.id > places.len() {
                    panic!(
                        "Place {}:{} undefined in stroke set {} for {}",
                        anchor_place.id,
                        anchor_place.at,
                        strokes.tag,
                        self.name
                    )
                } else if anchor_place.id == places.len() {
                    places.push(this_stroke.find_anchor(&anchor_place.at));
                } else {
                    let coord = places[anchor_place.id];
                    let anchor_coord = this_stroke.find_anchor(&anchor_place.at);
                    let difference = anchor_coord - coord;

                    this_stroke.translate(difference);
                }
            }

            stroke_points.push(this_stroke);
        }

        Scml::convert_svg(&stroke_points, &self.name)
    }

    fn convert_svg(strokes: &Vec<StrokeDescription>, name: &str) -> Result<String> {
        let mut document = Document::new().set("viewBox", (0, 0, 1, 1));

        for stroke in strokes.iter() {
            let mut path_data = Data::new().move_to(stroke.anchors[0].point);

            for anchor in &stroke.anchors[1..] {
                path_data = path_data.line_to(anchor.point);
            }

            let path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 0.01)
                .set("d", path_data);

            document = document.add(path);
        }

        Ok(document.to_string())
    }
}


impl StrokeDescription {
    fn top_left(&self) -> Point {
        let mut min_point = Point {
            x: f32::INFINITY,
            y: f32::INFINITY,
        };

        for anchor in self.anchors.iter() {
            min_point = min_point.min(anchor.point);
        }

        min_point
    }

    fn bottom_right(&self) -> Point {
        let mut max_point = Point {
            x: -f32::INFINITY,
            y: -f32::INFINITY,
        };

        for anchor in self.anchors.iter() {
            max_point = max_point.max(anchor.point);
        }

        max_point
    }

    pub fn translate(&mut self, point: Point) {
        for i in 0..self.anchors.len() {
            self.anchors[i].point -= point;
        }

        for i in 0..self.locations.len() {
            self.locations[i].direction -= point;
        }
    }

    pub fn scale(&mut self, size: f32) {
        for i in 0..self.anchors.len() {
            self.anchors[i].point *= size;
        }

        for i in 0..self.locations.len() {
            self.locations[i].direction *= size;
        }
    }

    pub fn scale_stretch(&mut self, scale: Point) {
        for i in 0..self.anchors.len() {
            self.anchors[i].point.x *= scale.x;
            self.anchors[i].point.y *= scale.y;
        }

        for i in 0..self.locations.len() {
            self.locations[i].direction.x *= scale.x;
            self.locations[i].direction.y *= scale.y;
        }
    }

    /// snaps to origin
    pub fn snap(&mut self) {
        let scale = self.top_left();
        self.translate(scale);
    }

    /// fits anchors to normal box
    pub fn fit(&mut self) {
        self.snap();

        // find maximum dimension
        let max_point = self.bottom_right();
        let max_dimension = max_point.x.max(max_point.y);

        self.scale(1.0 / max_dimension);
    }

    fn find_anchor(&self, name: &str) -> Point {
        for anchor in self.anchors.iter() {
            if anchor.name == name {
                return anchor.point.clone();
            }
        }
        panic!("Invalid anchor name {}", name);
    }
}

impl StrokeDictionary {
    fn get(&self, stroke_type: &str) -> Option<&StrokeDescription> {
        self.descriptions.get(stroke_type)
    }
}
