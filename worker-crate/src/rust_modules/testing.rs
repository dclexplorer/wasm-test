use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn op_log_test_result(result: JsValue) {
    console::log_1(&format!("Test result: {:?}", result).into());
}

#[wasm_bindgen]
pub fn op_log_test_plan(plan: JsValue) {
    console::log_1(&format!("Test plan: {:?}", plan).into());
}

#[wasm_bindgen]
pub async fn op_take_and_compare_snapshot(
    src_stored_snapshot: String,
    camera_position: Vec<f32>,
    camera_target: Vec<f32>,
    screenshot_size: Vec<f32>,
    methods: JsValue,
) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_take_and_compare_snapshot called").into());
    Ok(JsValue::from_bool(true))
}

// Add missing functions
#[wasm_bindgen]
pub async fn op_test_plan(plan: JsValue) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_test_plan called with plan: {:?}", plan).into());
    Ok(JsValue::UNDEFINED)
}

#[wasm_bindgen]
pub async fn op_test_snapshot(name: String, data: JsValue) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_test_snapshot called for test: {}", name).into());
    Ok(JsValue::UNDEFINED)
}

#[wasm_bindgen]
pub async fn op_test_result(
    name: String,
    success: bool,
    error: Option<String>,
    stack: Option<String>,
    total_frames: u32,
    total_time: f64,
) -> Result<JsValue, JsValue> {
    console::log_1(&format!("Test '{}' result - Success: {}", name, success).into());
    Ok(JsValue::UNDEFINED)
}

#[wasm_bindgen]
pub async fn testing_enabled(test: JsValue) -> Result<JsValue, JsValue> {
    Ok(JsValue::from_bool(false)) // Mock implementation
}
