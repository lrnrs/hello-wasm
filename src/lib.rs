use wasm_bindgen::prelude::*;
mod base;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let gen: String = base::gen_hello();
    let name_and_gen: String = name.to_string() + " " + &gen;
    let salutation: String = "Hello, ".to_string() + &name_and_gen;
    alert(&salutation);
}

#[wasm_bindgen]
pub fn rotate_shape(side: f32, orientation: f32) -> String {
    base::rotate_shape(side, orientation)
}

#[wasm_bindgen]
pub fn translate_shape(side: f32, x: f32, y: f32) -> String {
    base::translate_shape(side, x, y)
}
