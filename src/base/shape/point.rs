#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn distance_to(&self, other: &Point) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

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
