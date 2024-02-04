use crate::base::shape::{Path, Point, Shape};
use crate::base::Transformation;

pub struct Nest {
    transformations: Vec<Box<dyn Transformation>>,
}

impl Nest {
    pub fn from(transformations: Vec<Box<dyn Transformation>>) -> Nest {
        Nest { transformations }
    }
}

impl Transformation for Nest {
    fn apply(&self, s: Shape) -> Shape {
        let mut result: Shape = s.clone();
        for transformation in &self.transformations {
            result = transformation.apply(result);
        }
        s.add(&result);
        s
    }
}