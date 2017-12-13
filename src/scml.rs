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

    /// turns abstract structure into points
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

        Scml::normalize(&mut stroke_points);
        Scml::convert_svg(&stroke_points)
    }

    /// find upper left bounds of entire character
    fn top_left(strokes: &Vec<StrokeDescription>) -> Point {
        let mut min_point = Point {
            x: f32::INFINITY,
            y: f32::INFINITY,
        };

        for stroke in strokes.iter() {
            min_point = min_point.min(stroke.top_left());
        }

        min_point
    }

    /// find lower right bounds of entire character
    fn bottom_right(strokes: &Vec<StrokeDescription>) -> Point {
        let mut max_point = Point {
            x: -f32::INFINITY,
            y: -f32::INFINITY,
        };

        for stroke in strokes.iter() {
            max_point = max_point.max(stroke.bottom_right());
        }

        max_point
    }

    /// normalize character to fit within 1x1 square
    fn normalize(strokes: &mut Vec<StrokeDescription>) {
        let min_point = Scml::top_left(strokes);
        
        for i in 0..strokes.len() {
            strokes[i].translate(min_point);
        }

        let max_point = Scml::bottom_right(strokes);
        let max_dimension = max_point.x.max(max_point.y);

        for i in 0..strokes.len() {
            strokes[i].scale(1.0 / max_dimension)
        }
    }

    /// convert character into svg format
    fn convert_svg(strokes: &Vec<StrokeDescription>) -> Result<String> {
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
