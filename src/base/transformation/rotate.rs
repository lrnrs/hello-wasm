use crate::base::shape::{Path, Point, Shape};
use crate::base::Transformation;

pub struct Rotate {
    around: Option<Point>,
    radians: f32,
}

impl Rotate {
    pub fn from(degrees: f32) -> Rotate {
        let radians = Rotate::degrees_to_radians(degrees);
        Rotate { around: None, radians: radians }
    }

    fn rotated_path(&self, p: &Path, around: &Point) -> Path {
        let new_points: Vec<Point> = p.points.iter().map(|point| self.rotated_point(point, around)).collect();
        Path::from(new_points)
    }

    fn degrees_to_radians(degrees: f32) -> f32 {
        degrees * std::f32::consts::PI / 180.0
    }

    fn rotated_point(&self, p: &Point, around: &Point) -> Point {
        let new_x = (p.x - around.x) * self.radians.cos() - (p.y - around.y) * self.radians.sin();
        let new_y = (p.x - around.x) * self.radians.sin() + (p.y - around.y) * self.radians.cos();
        let rotated = Point::from(new_x, new_y);
        rotated
    }
}

impl Transformation for Rotate {
    fn apply(&self, s: Shape) -> Shape {
        let around = self.around.clone().unwrap_or(s.center());
        let new_paths: Vec<Path> = s.paths.iter().map(|path| self.rotated_path(path, &around)).collect();
        Shape::from(new_paths)
    }
}

