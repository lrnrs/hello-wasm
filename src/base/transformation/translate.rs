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

    fn a_point(&self, p: &Point) -> Point {
        let new_x: f32 = p.x + self.direction.x * self.distance;
        let new_y: f32 = p.y + self.direction.y * self.distance;
        Point::from(new_x, new_y)
    }

    fn a_path(&self, p: &Path) -> Path {
        let new_points: Vec<Point> = p.points.iter().map(|point| self.a_point(point)).collect();
        Path::from(new_points)
    }
}

impl Transformation for Translate {
    fn apply(&self, s: Shape) -> Shape {
        let new_paths: Vec<Path> = s.paths.iter().map(|path| self.a_path(path)).collect();
        Shape::from(new_paths)
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
    fn it_translates_a_point() {
        let t = Translate::from(Point::from(1.0, 1.0), 1.0);
        let p = Point::from(1.0, 1.0);
        let a = t.a_point(&p);

        test(&a, &Point::from(1.7071068, 1.7071068));
    }

    #[test]
    fn it_translates_a_path() {
        let t = Translate::from(Point::from(1.0, 1.0), 1.0);
        let p = Path::line(Point::from(1.0, 1.0), Point::from(2.0, 2.0));
        let a = t.a_path(&p);

        test(&a.points[0], &Point::from(1.7071068, 1.7071068));
        test(&a.points[1], &Point::from(2.7071068, 2.7071068));
    }

    #[test]
    fn it_translates_a_shape() {
        let t = Translate::from(Point::from(1.0, 1.0), 1.0);
        let p = Path::line(Point::from(1.0, 1.0), Point::from(2.0, 2.0));
        let s = Shape::from(vec![p]);
        let a = t.apply(s);

        test(&a.paths[0].points[0], &Point::from(1.7071068, 1.7071068));
        test(&a.paths[0].points[1], &Point::from(2.7071068, 2.7071068));
    }

    #[test]
    fn it_is_reversible_for_paths() {
        let translate = Translate::from(Point::from(std::f32::consts::PI, std::f32::consts::PI), std::f32::consts::PI);
        let reverse = Translate::from(Point::from(std::f32::consts::PI, std::f32::consts::PI), -std::f32::consts::PI);
        let p = Path::line(Point::from(1.0, 1.0), Point::from(2.0, 2.0));
        let translated = translate.a_path(&p);
        let reversed = reverse.a_path(&translated);

        test(&reversed.points[0], &p.points[0]);
        test(&reversed.points[1], &p.points[1]);
    }

    #[test]
    fn it_is_reversible_for_shapes() {
        let translate = Translate::from(Point::from(1.0, 1.0), 1.0);
        let reverse = Translate::from(Point::from(1.0, 1.0), -1.0);

        let shape = Shape::from(vec![Path::line(Point::from(1.0, 1.0), Point::from(2.0, 2.0))]);
        let translated = translate.apply(shape.clone());
        let reversed = reverse.apply(translated);

        test(&reversed.paths[0].points[0], &shape.paths[0].points[0]);
        test(&reversed.paths[0].points[1], &shape.paths[0].points[1]);
    }

    #[test]
    fn it_has_noop() {
        let translate = Translate::from(Point::from(0.0, 1.0), 0.0);
        let p = Path::line(Point::from(1.0, 1.0), Point::from(2.0, 2.0));
        let s = Shape::from(vec![p]);
        let translated = translate.apply(s.clone());

        test(&translated.paths[0].points[0], &s.paths[0].points[0]);
        test(&translated.paths[0].points[1], &s.paths[0].points[1]);
    }
}

