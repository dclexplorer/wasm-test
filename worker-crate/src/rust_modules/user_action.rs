use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(js_name = "op_teleport_to")]
pub async fn teleport_to(x: i32, y: i32) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_teleport_to called with x: {}, y: {}", x, y).into());
    Ok(JsValue::from_str("Teleported successfully"))
}

#[wasm_bindgen(js_name = "op_show_alert")]
pub fn show_alert(title: String, message: String) {
    console::log_1(&format!("Showing alert - Title: {}, Message: {}", title, message).into());
}

#[wasm_bindgen(js_name = "op_subscribe")]
pub async fn subscribe(event_type: String) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_subscribe called for event type: {}", event_type).into());
    Ok(JsValue::from_str(&format!("subscription-{}", event_type)))
}

#[wasm_bindgen(js_name = "op_unsubscribe")]
pub async fn unsubscribe(event_id: String) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_unsubscribe called for event: {}", event_id).into());
    Ok(JsValue::UNDEFINED)
}
