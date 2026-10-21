use wasm_bindgen::prelude::*;

#[path = "sorting/sorting_visualization/mod.rs"]
mod sorting_visualization;

#[wasm_bindgen]
pub fn sort_with_trace(
    algorithm: &str,
    values: Vec<i32>,
) -> Result<JsValue, JsValue> {
    let result = sorting_visualization::sort_with_trace(algorithm, values);

    serde_wasm_bindgen::to_value(&result)
        .map_err(|error| JsValue::from_str(&error.to_string()))
}