use crate::base::shape::{Path, Point, Shape};
use crate::base::Transformation;

#[derive(Debug)]
pub struct Rotate {
    around: Option<Point>,
    radians: f32,
}

impl Rotate {
    pub fn from(degrees: f32) -> Rotate {
        let radians = Rotate::degrees_to_radians(degrees);
        Rotate { around: None, radians }
    }

    pub fn around(degrees: f32, p: Point) -> Rotate {
        let radians = Rotate::degrees_to_radians(degrees);
        Rotate { around: Some(p), radians }
    }

    fn a_point(&self, p: &Point, around: &Point) -> Point {
        let new_x: f32 = around.x + (p.x - around.x) * self.radians.cos() - (p.y - around.y) * self.radians.sin();
        let new_y: f32 = around.y + (p.x - around.x) * self.radians.sin() + (p.y - around.y) * self.radians.cos();
        Point::from(new_x, new_y)
    }

    fn a_path(&self, p: &Path, around: &Point) -> Path {
        let new_points: Vec<Point> = p.points.iter().map(|point| self.a_point(point, around)).collect();
        Path::from(new_points)
    }

    fn a_shape(&self, s: &Shape, around: &Point) -> Shape {
        let new_paths: Vec<Path> = s.paths.iter().map(|path| self.a_path(path, &around)).collect();
        Shape::from(new_paths)
    }

    fn degrees_to_radians(degrees: f32) -> f32 {
        let normalized_degree = degrees % 360.0;
        normalized_degree * std::f32::consts::PI / 180.0
    }
}

impl Transformation for Rotate {
    fn apply(&self, s: Shape) -> Shape {
        let around = self.around.clone().unwrap_or(s.center());
        self.a_shape(&s, &around)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const PRECISION: f32 = 0.00001;
    fn test(expected: &Point, result: &Point) {
        assert!(expected.distance_to(result) < PRECISION, "Expected {:?}, got {:?}", expected, result);
    }

    #[test]
    fn it_rotates_a_point() {
        let r = Rotate::from(90.0);
        let p = Point::from(1.0, 0.0);
        let rotated = r.a_point(&p, &Point::from(0.0, 0.0));

        test(&Point::from(0.0, 1.0), &rotated);
    }

    #[test]
    fn it_is_reversible_for_points() {
        let r1 = Rotate::from(90.0);
        let r2 = Rotate::from(-90.0);
        let p = Point::from(1.0, 0.0);
        let rotated = r1.a_point(&p, &Point::from(0.0, 0.0));
        let inverted = r2.a_point(&rotated, &Point::from(0.0, 0.0));

        test(&inverted, &p);
    }

    #[test]
    fn it_rotates_a_path() {
        let r = Rotate::from(90.0);
        let p = Path::line(Point::from(1.0, 0.0), Point::from(2.0, 0.0));
        let a = r.a_path(&p, &Point::from(0.0, 0.0));

        test(&Point::from(0.0, 1.0), &a.points[0]);
        test(&Point::from(0.0, 2.0), &a.points[1]);
    }

    #[test]
    fn it_is_reversible_for_paths() {
        let rotate = Rotate::from(90.0);
        let invert = Rotate::from(-90.0);
        let p = Path::line(Point::from(1.0, 0.0), Point::from(2.0, 0.0));
        let rotated = rotate.a_path(&p, &Point::from(0.0, 0.0));
        let inverted = invert.a_path(&rotated, &Point::from(0.0, 0.0));
        
        test(&p.points[0], &inverted.points[0]);
        test(&p.points[1], &inverted.points[1]);
    }

    #[test]
    fn it_rotates_a_shape() {
        let corner_1 = Point::from(-1.0, -1.0);
        let corner_2 = Point::from(1.0, -1.0);
        let corner_3 = Point::from(1.0, 1.0);
        let corner_4 = Point::from(-1.0, 1.0);
        let edge_1 = Path::line(corner_1.clone(), corner_2.clone());
        let edge_2 = Path::line(corner_2, corner_3.clone());
        let edge_3 = Path::line(corner_3, corner_4.clone());
        let edge_4 = Path::line(corner_4, corner_1);
        let shape = Shape::from(vec![edge_1, edge_2, edge_3, edge_4]);

        let r = <dyn Transformation>::rotate(90.0);
        let rotated = r.apply(shape);

        let rotated_corner_1 = Point::from(1.0, -1.0);
        let rotated_corner_2 = Point::from(1.0, 1.0);
        let rotated_corner_3 = Point::from(-1.0, 1.0);
        let rotated_corner_4 = Point::from(-1.0, -1.0);

        test(&rotated_corner_1, &rotated.paths[0].points[0]);
        test(&rotated_corner_2, &rotated.paths[0].points[1]);
        test(&rotated_corner_2, &rotated.paths[1].points[0]);
        test(&rotated_corner_3, &rotated.paths[1].points[1]);
        test(&rotated_corner_3, &rotated.paths[2].points[0]);
        test(&rotated_corner_4, &rotated.paths[2].points[1]);
        test(&rotated_corner_4, &rotated.paths[3].points[0]);
        test(&rotated_corner_1, &rotated.paths[3].points[1]);
    }

    #[test]
    fn it_is_reversible_for_shapes() {
        let rotate = Rotate::from(90.0);
        let invert = Rotate::from(-90.0);
        let paths = vec![
            Path::line(Point::from(-1.0, -1.0), Point::from(1.0, -1.0)),
            Path::line(Point::from(1.0, -1.0), Point::from(1.0, 1.0)),
            Path::line(Point::from(1.0, 1.0), Point::from(-1.0, 1.0)),
            Path::line(Point::from(-1.0, 1.0), Point::from(-1.0, -1.0))
        ];

        let shape = Shape::from(paths);
        let rotated = rotate.a_shape(&shape, &Point::from(0.0, 0.0));
        let inverted = invert.a_shape(&rotated, &Point::from(0.0, 0.0));

        test(&shape.paths[0].points[0], &inverted.paths[0].points[0]);
        test(&shape.paths[0].points[1], &inverted.paths[0].points[1]);
        test(&shape.paths[0].points[0], &inverted.paths[0].points[0]);
    }

    #[test]
    fn it_has_noop() {
        let r = Rotate::around(0.0, Point::from(1.0, 1.0));
        let p = Point::from(0.0, 0.0);
        let shape = Shape::from(vec![Path::from(vec![p.clone()])]);
        let rotated = r.apply(shape.clone());

        test(&Point::from(0.0, 0.0), &rotated.paths[0].points[0]);
    }

    #[test]
    fn it_turns_around() {
        let r = Rotate::around(360.0, Point::from(1.0, 1.0));
        let p = Point::from(0.0, 0.0);
        let shape = Shape::from(vec![Path::from(vec![p.clone()])]);
        let rotated = r.apply(shape.clone());

        test(&Point::from(0.0, 0.0), &rotated.paths[0].points[0]);
    }
}
