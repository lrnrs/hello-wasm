use crate::base::shape::path::Path;
use crate::base::shape::point::Point;
use crate::base::shape::shape::Shape;
use crate::base::Transformation;

pub struct Scale {
    factor: f32,
}

impl Scale {
    pub fn from(factor: f32) -> Scale {
        Scale { factor }
    }

    fn scaled_path(&self, p: &Path) -> Path {
        let new_points: Vec<Point> = p
            .points
            .iter()
            .map(|point| self.scaled_point(point))
            .collect();
        Path::from(new_points)
    }

    fn scaled_point(&self, p: &Point) -> Point {
        let new_x = p.x * self.factor;
        let new_y = p.y * self.factor;
        Point::from(new_x, new_y)
    }
}

impl Transformation for Scale {
    fn apply(&self, s: Shape) -> Shape {
        let new_paths: Vec<Path> = s.paths.iter().map(|path| self.scaled_path(path)).collect();
        Shape::from(new_paths)
    }
}
