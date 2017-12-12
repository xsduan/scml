use std::f32;
use std::ops::*;
use std::str;

use parse::*;

impl Clone for Point {
    fn clone(&self) -> Point {
        *self
    }
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

impl Mul<f32> for Point {
    type Output = Point;
    fn mul(self, other: f32) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl MulAssign<f32> for Point {
    fn mul_assign(&mut self, other: f32) {
        *self = Point {
            x: self.x * other,
            y: self.y * other,
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

pub fn transform(character: Scml) {
    // TODO: implement
}
