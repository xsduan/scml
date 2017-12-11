use std::f32;
use std::ops::*;
use std::str;

use parse::*;

impl Clone for Point {
    fn clone(&self) -> Point { *self }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Point) {
        *self = Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Div<f32> for Point {
    type Output = Point;
    fn div(self, other: f32) -> Point {
        Point {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl DivAssign<f32> for Point {
    fn div_assign(&mut self, other: f32) {
        *self = Point {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Point {
    fn min(self, other: Point) -> Point {
        Point {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    fn max(self, other: Point) -> Point {
        Point {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
}

impl StrokeDescription {
    fn top_left(&self) -> Point {
        let mut min_point = Point { x: f32::INFINITY, y: f32::INFINITY };

        for anchor in self.anchors.iter() {
            min_point = min_point.min(anchor.point);
        }

        min_point
    }

    fn bottom_right(&self) -> Point {
        let mut max_point = Point { x: -f32::INFINITY, y: -f32::INFINITY };

        for anchor in self.anchors.iter() {
            max_point = max_point.max(anchor.point);
        }

        max_point
    }

    /// snaps to origin
    pub fn snap(&mut self) {
        let min_point = self.top_left();

        for i in 0..self.anchors.len() {
            self.anchors[i].point -= min_point;
        }

        for i in 0..self.locations.len() {
            self.locations[i].direction -= min_point;
        }
    }

    /// fits stroke into normal box regardless of top left boundaries
    fn scale_normal(&mut self) {
        // find maximum dimension
        let max_point = self.bottom_right();
        let max_dimension = max_point.x.max(max_point.y);

        for i in 0..self.anchors.len() {
            self.anchors[i].point /= max_dimension;
        }

        for i in 0..self.locations.len() {
            self.locations[i].direction /= max_dimension;
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
