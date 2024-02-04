use crate::base::shape::{Path, Point, Shape};
use crate::base::Transformation;

pub struct Translate {
    direction: Point,
    distance: f32,
}

impl Translate {
    pub fn from(direction: Point, distance: f32) -> Translate {
        Translate { direction: direction.normalized(), distance }
    }

    fn translated_point(&self, p: &Point) -> Point {
        let new_x: f32 = p.x + self.direction.x * self.distance;
        let new_y: f32 = p.y + self.direction.y * self.distance;
        Point::from(new_x, new_y)
    }

    fn translated_path(&self, p: &Path) -> Path {
        let new_points: Vec<Point> = p.points.iter().map(|point| self.translated_point(point)).collect();
        Path::from(new_points)
    }
}

impl Transformation for Translate {
    fn apply(&self, s: Shape) -> Shape {
        let new_paths: Vec<Path> = s.paths.iter().map(|path| self.translated_path(path)).collect();
        Shape::from(new_paths)
    }
}

pub struct Scale {
    factor: f32,
}

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