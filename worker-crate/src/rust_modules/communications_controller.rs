use wasm_bindgen::prelude::*;
use js_sys::{Array, Uint8Array};
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_comms_send_string)]
    pub async fn js_comms_send_string(message: String, unused: String) -> Result<(), JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_comms_send_binary_single)]
    pub async fn js_comms_send_binary_single(data: Vec<u8>, address: Option<String>) -> Result<(), JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_comms_recv_binary)]
    pub async fn js_comms_recv_binary() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_comms_adapter_send_chat)]
    pub async fn js_comms_adapter_send_chat(message: String, channel: String) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_comms_send_binary)]
    pub async fn js_comms_send_binary(data: Vec<u8>) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_comms_send_chat)]
    pub async fn js_comms_send_chat(message: String) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen(js_name = "op_comms_send_string")]
pub async fn comms_send_string(message: String, _unused: String) -> Result<(), JsValue> {
    console::log_1(&format!("Sending string message: {}", message).into());
    Ok(())
}

#[wasm_bindgen(js_name = "op_comms_send_binary_single")]
pub async fn comms_send_binary_single(data: Vec<u8>, address: Option<String>) -> Result<(), JsValue> {
    console::log_1(&format!("Sending binary data ({} bytes) to {:?}", data.len(), address).into());
    Ok(())
}

#[wasm_bindgen(js_name = "op_comms_recv_binary")]
pub async fn comms_recv_binary() -> Result<JsValue, JsValue> {
    console::log_1(&"Receiving binary data".into());
    Ok(Array::new().into())
}

// Add missing functions
#[wasm_bindgen(js_name = "op_comms_adapter_send_chat")]
pub async fn adapter_send_chat(message: String, channel: String) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_comms_adapter_send_chat called with message: {} on channel: {}", message, channel).into());
    Ok(JsValue::UNDEFINED)
}

#[wasm_bindgen(js_name = "op_comms_send_binary")]
pub async fn send_binary(data: Vec<u8>) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_comms_send_binary called with {} bytes", data.len()).into());
    Ok(JsValue::UNDEFINED)
}

#[wasm_bindgen(js_name = "op_comms_send_chat")]
pub async fn send_chat(message: String) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_comms_send_chat called with message: {}", message).into());
    Ok(JsValue::UNDEFINED)
}
