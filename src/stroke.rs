use std::f32;
use std::str;

use parse::*;
use point::*;

impl StrokeDescription {
    /// find upper left bounds
    pub fn top_left(&self) -> Point {
        let mut min_point = Point {
            x: f32::INFINITY,
            y: f32::INFINITY,
        };

        for anchor in self.anchors.iter() {
            min_point = min_point.min(anchor.point);
        }

        min_point
    }

    /// find lower right bounds
    pub fn bottom_right(&self) -> Point {
        let mut max_point = Point {
            x: -f32::INFINITY,
            y: -f32::INFINITY,
        };

        for anchor in self.anchors.iter() {
            max_point = max_point.max(anchor.point);
        }

        max_point
    }

    /// translate entire stroke
    pub fn translate(&mut self, point: Point) {
        for i in 0..self.anchors.len() {
            self.anchors[i].point -= point;
        }

        for i in 0..self.locations.len() {
            self.locations[i].direction -= point;
        }
    }

    /// stretch axes proportionally
    pub fn scale(&mut self, size: f32) {
        for i in 0..self.anchors.len() {
            self.anchors[i].point *= size;
        }

        for i in 0..self.locations.len() {
            self.locations[i].direction *= size;
        }
    }

    /// stretches axes independently
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

        let max_point = self.bottom_right();
        let max_dimension = max_point.x.max(max_point.y);

        self.scale(1.0 / max_dimension);
    }

    /// find anchor by name
    pub fn find_anchor(&self, name: &str) -> Point {
        for anchor in self.anchors.iter() {
            if anchor.name == name {
                return anchor.point.clone();
            }
        }
        panic!("Invalid anchor name {}", name);
    }
}

impl StrokeDictionary {
    /// get stroke by name
    pub fn get(&self, stroke_type: &str) -> Option<&StrokeDescription> {
        self.descriptions.get(stroke_type)
    }
}
