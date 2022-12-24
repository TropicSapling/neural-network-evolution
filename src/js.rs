use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	pub fn log(s: &str);

	#[wasm_bindgen(js_namespace = window)]
	pub fn draw_bg();

	#[wasm_bindgen(js_namespace = window)]
	pub fn draw_agent(r: usize, g: usize, b: usize, x: f64, y: f64, size: f64);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
