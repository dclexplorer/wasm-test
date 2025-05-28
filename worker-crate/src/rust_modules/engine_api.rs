use wasm_bindgen::prelude::*;
use web_sys::console;
use js_sys::Array;

#[wasm_bindgen(js_name = "op_crdt_send_to_renderer")]
pub fn crdt_send_to_renderer(data: &[u8]) {
    console::log_1(&format!("Sending {} bytes to renderer", data.len()).into());
}

#[wasm_bindgen(js_name = "op_crdt_recv_from_renderer")]
pub fn crdt_recv_from_renderer() -> Vec<u8> {
    console::log_1(&"Receiving from renderer".into());
    vec![]
}

#[wasm_bindgen(js_name = "op_subscribe")]
pub fn subscribe(event_id: u32) {
    console::log_1(&format!("Subscribing to event {}", event_id).into());
}

#[wasm_bindgen(js_name = "op_unsubscribe")]
pub fn unsubscribe(event_id: u32) {
    console::log_1(&format!("Unsubscribing from event {}", event_id).into());
}

#[wasm_bindgen(js_name = "op_send_batch")]
pub fn send_batch() -> js_sys::Array {
    console::log_1(&"op_send_batch called".into());
    js_sys::Array::new() // Return empty events array
}

#[wasm_bindgen(js_name = "op_get_visible_peers")]
pub async fn get_visible_peers() -> Result<JsValue, JsValue> {
    console::log_1(&"op_get_visible_peers called".into());
    let array = Array::new();
    Ok(array.into())
}

// Register all ops for this module
pub fn register_ops(ops: &js_sys::Object) -> Result<(), JsValue> {
    use js_sys::Reflect;
    
    Reflect::set(ops, &"op_crdt_send_to_renderer".into(), &crdt_send_to_renderer.into())?;
    Reflect::set(ops, &"op_crdt_recv_from_renderer".into(), &crdt_recv_from_renderer.into())?;
    Reflect::set(ops, &"op_crdt_recv_from_renderer_legacy".into(), &crdt_recv_from_renderer_legacy.into())?;
    Reflect::set(ops, &"op_get_visible_peers".into(), &get_visible_peers.into())?;
    
    Ok(())
}
