use crate::base::shape::path::Path;
use crate::base::shape::point::Point;
use crate::base::transformation::transformation::Transformation;

#[derive(Clone, Debug)]
pub struct Shape {
    pub paths: Vec<Path>,
}

impl Shape {
    pub fn add(&self, other: &Shape) -> Shape {
        let existing_paths = &self.paths;
        let other_paths = &other.paths;
        let all_paths = existing_paths.iter().chain(other_paths.iter());
        let mut new_paths: Vec<Path> = vec![];
        for path in all_paths {
            new_paths.push(path.clone());
        }
        Shape::from(new_paths)
    }

    pub fn from(paths: Vec<Path>) -> Shape {
        Shape { paths }
    }

    pub fn center(&self) -> Point {
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        let mut n: f32 = 0.0;
        for path in &self.paths {
            for point in &path.points {
                x += point.x;
                y += point.y;
                n += 1.0;
            }
        }
        Point::from(x / n, y / n)
    }

    pub fn square(side: f32, transformations: Option<Vec<impl Transformation>>) -> Shape {
        let half_side: f32 = side / 2.0;
        let paths: Vec<Path> = vec![
            Path::line(
                Point::from(-half_side, -half_side),
                Point::from(half_side, -half_side),
            ),
            Path::line(
                Point::from(half_side, -half_side),
                Point::from(half_side, half_side),
            ),
            Path::line(
                Point::from(half_side, half_side),
                Point::from(-half_side, half_side),
            ),
            Path::line(
                Point::from(-half_side, half_side),
                Point::from(-half_side, -half_side),
            ),
        ];
        let shape = Shape::from(paths);

        match transformations {
            Some(ts) => {
                if ts.len() > 0 {
                    let mut new_shape: Shape = shape.clone();
                    for t in ts {
                        new_shape = t.apply(new_shape);
                    }
                    new_shape
                } else {
                    shape
                }
            }
            None => shape,
        }
    }

    pub fn line(length: f32, transformations: Option<Vec<impl Transformation>>) -> Shape {
        let line = Path::line(Point::from(0.0, 0.0), Point::from(length, 0.0));
        let shape = Shape::from(vec![line]);
        match transformations {
            Some(ts) => {
                if ts.len() > 0 {
                    let mut new_shape = shape.clone();
                    for t in ts {
                        new_shape = t.apply(new_shape);
                    }
                    new_shape
                } else {
                    shape
                }
            }
            None => shape,
        }
    }

    fn centered_on(&self, p: Point) -> Shape {
        let center = self.center();
        let dx = p.x - center.x;
        let dy = p.y - center.y;
        let new_paths: Vec<Path> = self
            .paths
            .iter()
            .map(|path| {
                let new_points: Vec<Point> = path
                    .points
                    .iter()
                    .map(|point| Point::from(point.x + dx, point.y + dy))
                    .collect();
                Path::from(new_points)
            })
            .collect();
        Shape::from(new_paths)
    }

    pub fn to_svg(&self) -> String {
        let dim_x = 200;
        let dim_y = 200;
        let center = Point::from((dim_x as f32) / 2.0, (dim_y as f32) / 2.0);
        let centered = self.centered_on(center);
        let mut svg: String = "<svg width=\"".to_string()
            + &dim_x.to_string()
            + "\" height=\""
            + &dim_y.to_string()
            + "\" xmlns=\"http://www.w3.org/2000/svg\">";
        for path in centered.paths {
            svg = svg + "<path d=\"";
            svg = svg
                + "M"
                + &path.points[0].x.to_string()
                + " "
                + &path.points[0].y.to_string()
                + " ";
            for point in path.points.iter().skip(1) {
                svg = svg + "L" + &point.x.to_string() + " " + &point.y.to_string() + " ";
            }
            svg = svg + "Z\" style=\"fill:none;stroke:black;stroke-width:2\"/>";
        }
        svg = svg + "</svg>";
        svg
    }
}
