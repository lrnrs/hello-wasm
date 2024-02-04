use crate::base::shape::{Point, Shape};
use crate::base::transformation::translate::Translate;
use crate::base::transformation::rotate::Rotate;

pub trait Transformation {
    fn apply(&self, s: Shape) -> Shape;
}

impl dyn Transformation {
    pub fn rotate(degrees: f32) -> impl Transformation {
        Rotate::from(degrees)
    }
    pub fn translate(x: f32, y: f32) -> impl Transformation {
        let p: Point = Point::from(x, y);
        Translate::from(p.normalized(), p.length())
    }
}

