use std::f32;
use std::str;

use parse::Scml;

#[derive(Debug, Copy)]
pub struct Point(f32, f32);

impl Clone for Point {
    fn clone(&self) -> Point { *self }
}

// #[derive(Debug)]
// pub struct Location(pub f32, pub f32, pub String);

#[derive(Debug)]
pub struct Stroke {
    anchors: Vec<(Point, String)>,
    locations: Vec<(usize, usize, Point, String)>
}

impl Stroke {
    pub fn new(anchors: Vec<(Point, String)>, locations: Vec<(usize, usize, Point, String)>) -> Stroke {
        let mut stroke = Stroke {
            anchors: anchors,
            locations: locations,
        };

        stroke
    }

    fn top_left(&self) -> Point {
        let mut min_point = Point(f32::INFINITY, f32::INFINITY);

        for point in self.anchors.iter() {
            min_point.0 = min_point.0.min((point.0).0);
            min_point.1 = min_point.1.min((point.0).1);
        }

        min_point
    }

    fn bottom_right(&self) -> Point {
        let mut max_point = Point(-f32::INFINITY, -f32::INFINITY);

        for point in self.anchors.iter() {
            max_point.0 = max_point.0.max((point.0).0);
            max_point.1 = max_point.1.max((point.0).1);
        }

        max_point
    }

    /// snaps to origin
    pub fn snap(&mut self) {
        let min_point = self.top_left();

        for i in 0..self.anchors.len() {
            (self.anchors[i].0).0 -= min_point.0;
            (self.anchors[i].0).1 -= min_point.1;
        }

        for i in 0..self.locations.len() {
            (self.locations[i].2).0 -= min_point.0;
            (self.locations[i].2).0 -= min_point.1;
        }
    }

    /// fits stroke into normal box regardless of top left boundaries
    fn scale_normal(&mut self) {
        // find maximum dimension
        let max_point = self.bottom_right();
        let max_dimension = max_point.0.max(max_point.1);

        for i in 0..self.anchors.len() {
            (self.anchors[i].0).0 /= max_dimension;
            (self.anchors[i].0).1 /= max_dimension;
        }

        for i in 0..self.locations.len() {
            (self.locations[i].2).0 /= max_dimension;
            (self.locations[i].2).1 /= max_dimension;
        }
    }

    /// fits anchors to normal box
    pub fn fit(&mut self) {
        self.snap();
        self.scale_normal();
    }

    fn find_anchor(&self, name: &str) -> Point {
        let index = name.split("inside").last().expect("Invalid inside anchor").parse();

        match index {
            Ok(res) => { return self.inside(res); }
            Err(_err) => {
                for anchor in self.anchors.iter() {
                    if anchor.1 == name {
                        return anchor.0.clone();
                    }
                }
                panic!("Invalid anchor name {}", name);
            }
        }
    }

    fn inside(&self, i: usize) -> Point {
        // TODO: support for bézier averages
        if i >= self.anchors.len() - 1 {
            panic!("Invalid anchor name inside{}", i);
        }

        Point(
            ((self.anchors[i].0).0 + (self.anchors[i + 1].0).0) / 2.0,
            ((self.anchors[i].0).1 + (self.anchors[i + 1].0).1) / 2.0
        )
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
    // hard-coded s, hz, h
    let s = Stroke::new(vec![(Point(0.0, 0.0), "begin".to_string()), (Point(0.0, 1.0), "end".to_string())],
                        vec![]);
    let hz = Stroke::new(vec![(Point(0.0, 0.0), "begin".to_string()), (Point(1.0, 0.0), "corner".to_string()), (Point(1.0, 1.0), "end".to_string())],
                         vec![]);
    let h = Stroke::new(vec![(Point(0.0, 0.0), "begin".to_string()), (Point(1.0, 0.0), "end".to_string())],
                        vec![]);

    println!("{:#?}", hz.find_anchor("inside1"));
}
