pub mod shape;
pub mod transformation;

use shape::Shape;
use transformation::transformation::Transformation;

pub fn gen_hello() -> String {
    "Hello from base".to_string()
}

pub fn rotate_shape(side: f32, orientation_in_degrees: f32) -> String {
    let transformation = <dyn Transformation>::rotate(orientation_in_degrees); 
    let shape: Shape = Shape::square(side, transformation);
    shape.to_svg()
}

pub fn translate_shape(side: f32, x: f32, y: f32) -> String {
    let transformation = <dyn Transformation>::translate(x, y); 
    let shape: Shape = Shape::square(side, transformation);
    shape.to_svg()
}
