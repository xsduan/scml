use std::f32;
use std::str;

use svg::*;
use svg::node::element::Path;
use svg::node::element::path::Data;

use parse::*;
use point::*;

impl Scml {
    pub fn stroke_count(&self) -> usize {
        self.strokes.len()
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

pub fn transform(character: &Scml, strokes: &StrokeDictionary) {
    let mut stroke_points = Vec::with_capacity(character.stroke_count());
    let mut places = Vec::new();

    for stroke in character.strokes.iter() {
        // store copy of default, normalized stroke
        let this_stroke = strokes.get(&stroke.stroke_type);

        match this_stroke {
            Some(thing) => stroke_points.push(thing.clone()),
            None => {
                panic!(
                    "Invalid stroke type {} for {}",
                    stroke.stroke_type,
                    strokes.tag
                )
            }
        }
    }

    convert_svg(&stroke_points);
}

fn convert_svg(strokes: &Vec<StrokeDescription>) {
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

    save("image.svg", &document).unwrap();
}
