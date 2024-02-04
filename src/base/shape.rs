use crate::base::transformation::transformation::Transformation;

#[derive(Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn from(x: f32, y: f32) -> Point {
        Point { x, y }
    }
    pub fn normalized(&self) -> Point {
        let length = (self.x * self.x + self.y * self.y).sqrt();
        Point::from(self.x / length, self.y / length)
    }
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

#[derive(Clone)]
pub struct Path {
    pub points: Vec<Point>,
}

impl Path {
    pub fn from(points: Vec<Point>) -> Path {
        Path { points }
    }

    fn line(p1: Point, p2: Point) -> Path {
        Path {
            points: vec![p1, p2],
        }
    }
}
#[derive(Clone)]
pub struct Shape {
    pub paths: Vec<Path>
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

    pub fn square(side: f32, transformation: impl Transformation) -> Shape {
        let half_side = side / 2.0;
        let lines: Vec<Path> = vec![
            Path::line(Point::from(-half_side, -half_side), Point::from(half_side, -half_side)),
            Path::line(Point::from(half_side, -half_side), Point::from(half_side, half_side)),
            Path::line(Point::from(half_side, half_side), Point::from(-half_side, half_side)),
            Path::line(Point::from(-half_side, half_side), Point::from(-half_side, -half_side)),
        ];
        let shape = Shape::from(lines);
        transformation.apply(shape)
    }

    pub fn horizontal_line(length: f32) -> Shape {
        let line: Vec<Path> = vec![
            Path::line(Point::from(0.0, 0.0), Point::from(length, 0.0)),
        ];
        Shape::from(line)
    }

    pub fn vertical_line(length: f32) -> Shape {
        let line: Vec<Path> = vec![
            Path::line(Point::from(0.0, 0.0), Point::from(0.0, length)),
        ];
        Shape::from(line)
    }

    fn centered_on(&self, p: Point) -> Shape {
        let center = self.center();
        let dx = p.x - center.x;
        let dy = p.y - center.y;
        let new_paths: Vec<Path> = self.paths.iter().map(|path| {
            let new_points: Vec<Point> = path.points.iter().map(|point| {
                Point::from(point.x + dx, point.y + dy)
            }).collect();
            Path::from(new_points)
        }).collect();
        Shape::from(new_paths)
    }

    pub fn to_svg(&self) -> String {
        let dim_x = 200;
        let dim_y = 200;
        let center = Point::from((dim_x as f32) / 2.0, (dim_y as f32) / 2.0);
        let centered = self.centered_on(center);
        let mut svg: String = "<svg width=\"".to_string() + &dim_x.to_string() + "\" height=\"" + &dim_y.to_string() + "\" xmlns=\"http://www.w3.org/2000/svg\">";
        for path in centered.paths {
            svg = svg + "<path d=\"";
            svg = svg + "M" + &path.points[0].x.to_string() + " " + &path.points[0].y.to_string() + " ";
            for point in path.points.iter().skip(1) {
                svg = svg + "L" + &point.x.to_string() + " " + &point.y.to_string() + " ";
            }
            svg = svg + "Z\" style=\"fill:none;stroke:black;stroke-width:2\"/>";
        }
        svg = svg + "</svg>";
        svg
    }
}
