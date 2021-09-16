use wasm_bindgen::prelude::*;
use serde::{Serialize,Deserialize};
use js_sys::Reflect::{get,set};

#[derive(Serialize,Deserialize)]
pub struct Data {
  a: i32,
  b: i32,
  result: i32
}

#[wasm_bindgen]
pub fn compute(data: JsValue) {
  let deserialized: Data = data.into_serde().unwrap();
  let a = deserialized.a;
  let b = deserialized.b;
  let result: i32 = a + b;
  set(&data, &"result".into(), &result.into());
}

