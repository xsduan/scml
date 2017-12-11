use std::f32;
use std::str;

use parse::*;

impl Clone for Point {
    fn clone(&self) -> Point { *self }
}

impl StrokeDescription {
    fn top_left(&self) -> Point {
        let mut min_point = Point { x: f32::INFINITY, y: f32::INFINITY };

        for anchor in self.anchors.iter() {
            min_point.x = min_point.x.min(anchor.point.x);
            min_point.y = min_point.y.min(anchor.point.y);
        }

        min_point
    }

    fn bottom_right(&self) -> Point {
        let mut max_point = Point { x: -f32::INFINITY, y: -f32::INFINITY };

        for anchor in self.anchors.iter() {
            max_point.x = max_point.x.max(anchor.point.x);
            max_point.y = max_point.y.max(anchor.point.y);
        }

        max_point
    }

    /// snaps to origin
    pub fn snap(&mut self) {
        let min_point = self.top_left();

        for i in 0..self.anchors.len() {
            self.anchors[i].point.x -= min_point.x;
            self.anchors[i].point.y -= min_point.y;
        }

        for i in 0..self.locations.len() {
            self.locations[i].direction.x -= min_point.x;
            self.locations[i].direction.y -= min_point.y;
        }
    }

    /// fits stroke into normal box regardless of top left boundaries
    fn scale_normal(&mut self) {
        // find maximum dimension
        let max_point = self.bottom_right();
        let max_dimension = max_point.x.max(max_point.y);

        for i in 0..self.anchors.len() {
            self.anchors[i].point.x /= max_dimension;
            self.anchors[i].point.y /= max_dimension;
        }

        for i in 0..self.locations.len() {
            self.locations[i].direction.x /= max_dimension;
            self.locations[i].direction.y /= max_dimension;
        }
    }

    /// fits anchors to normal box
    pub fn fit(&mut self) {
        self.snap();
        self.scale_normal();
    }

    fn find_anchor(&self, name: &str) -> Point {
        for anchor in self.anchors.iter() {
            if anchor.name == name {
                return anchor.point.clone();
            }
        }
        panic!("Invalid anchor name {}", name);
    }

    // pub fn find_location(&self, name: &str) -> [(f32, f32); 3] {
    //     for location in self.locations.iter() {
    //         if location.2 == name {
    //             return (location.0, location.1);
    //         }
    //     }
    //     panic!("Invalid location name {}", name);
    // }
}

pub fn transform(character: Scml) {
    // TODO: implement
}
