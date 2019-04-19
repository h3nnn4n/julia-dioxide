extern crate cfg_if;
extern crate rand;
extern crate wasm_bindgen;
extern crate web_sys;

mod julia;
mod utils;

use cfg_if::cfg_if;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn js_alert(name: &str) {
    alert(&format!("{}", name));
}

#[wasm_bindgen]
pub fn a_plus_b(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn init() {
    let canvas = utils::get_canvas();

    canvas.set_width(600);
    canvas.set_height(600);

    render_julia_set();
}

#[wasm_bindgen]
pub fn render_julia_set() {
    let canvas = utils::get_canvas();
    let context = utils::get_canvas_context();

    let real = utils::get_input_value_by_id("real");
    let imaginary = utils::get_input_value_by_id("imaginary");

    let width = canvas.width();
    let height = canvas.height();

    let c = julia::Complex::new(real, imaginary);
    let mut data = julia::get_julia_set(width, height, c);
    let data =
        web_sys::ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)
            .unwrap();

    context.put_image_data(&data, 0.0, 0.0).unwrap();
}
