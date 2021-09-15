use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(x: i32, y: i32) -> i32 {
  use web_sys::console;
  let answer = format!("adding {} and {} yields {}", x, y, x+y);
  console::log_1(&answer.into());
  x + y
}

#[wasm_bindgen]
pub fn sub(x: i32, y: i32) -> i32 {
  x - y
}
