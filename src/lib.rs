use wasm_bindgen::prelude::*;
mod base;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}


#[wasm_bindgen(getter_with_clone)]
 pub struct RenderedShape {
    pub name: String,
    pub svg: String
}

#[wasm_bindgen]
pub fn render_rotated(dim: f32, param: f32) -> RenderedShape {
    let svg: String = base::rotate_shape(dim, param);
    console::log_1(&format!("dim: {}, param: {}, svg: {}", dim, param, svg).into());
    RenderedShape {
        name: "rotated".to_string(),
        svg: svg
    }
}

#[wasm_bindgen]
pub fn render_translated(dim: f32, param: f32) -> RenderedShape {
    let svg: String = base::translate_shape(dim, param, param);
    RenderedShape {
        name: "translated".to_string(),
        svg: svg
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let gen: String = base::gen_hello();
    let name_and_gen: String = name.to_string() + " " + &gen;
    let salutation: String = "Hello, ".to_string() + &name_and_gen;
    alert(&salutation);
}
