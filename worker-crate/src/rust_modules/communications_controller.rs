use wasm_bindgen::prelude::*;
use js_sys::{Array, Uint8Array};
use web_sys::console;

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

#[wasm_bindgen(js_name = "op_comms_send_chat")]
pub async fn send_chat(message: String) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_comms_send_chat called with message: {}", message).into());
    Ok(JsValue::UNDEFINED)
}

// Register all ops for this module
pub fn register_ops(ops: &js_sys::Object) -> Result<(), JsValue> {
    use js_sys::Reflect;
    
    Reflect::set(ops, &"op_comms_adapter_send_chat".into(), &adapter_send_chat.into())?;
    Reflect::set(ops, &"op_comms_send_binary".into(), &send_binary.into())?;
    Reflect::set(ops, &"op_comms_send_chat".into(), &send_chat.into())?;
    
    Ok(())
}
