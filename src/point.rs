use std::f32;
use std::ops::*;
use std::str;

#[derive(Deserialize, Debug, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

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
    pub fn min(self, other: Point) -> Point {
        Point {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn max(self, other: Point) -> Point {
        Point {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
}