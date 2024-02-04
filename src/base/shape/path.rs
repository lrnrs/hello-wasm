use crate::base::shape::point::Point;

#[derive(Clone, Debug)]
pub struct Path {
    pub points: Vec<Point>,
}

impl Path {
    pub fn from(points: Vec<Point>) -> Path {
        Path { points }
    }

    pub fn line(p1: Point, p2: Point) -> Path {
        Path {
            points: vec![p1, p2],
        }
    }
}
