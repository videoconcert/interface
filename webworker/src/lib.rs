use wasm_bindgen::prelude::*;

pub mod agents;

#[wasm_bindgen]
pub fn run_worker() -> Result<(), JsValue> {
  yew::initialize();
  Ok(())
}