use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(js_name = "op_request_ethereum_controller")]
pub async fn request_ethereum_controller() -> Result<JsValue, JsValue> {
    console::log_1(&"Requesting Ethereum controller".into());
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"message".into(), &"Ethereum controller granted".into())?;
    Ok(obj.into())
}

#[wasm_bindgen(js_name = "op_convert_message_to_object")]
pub async fn convert_message_to_object(message: String) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_convert_message_to_object called with message: {}", message).into());
    let obj = Object::new();
    Reflect::set(&obj, &"message".into(), &JsValue::from_str(&message))?;
    Ok(obj.into())
}

#[wasm_bindgen(js_name = "op_send_async")]
pub async fn send_async(method: String, json_params: String) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_send_async called with method: {}, params: {}", method, json_params).into());
    
    // Return mock response
    match method.as_str() {
        "eth_requestAccounts" => Ok(JsValue::from_str("[\"0x1234567890abcdef\"]")),
        "eth_blockNumber" => Ok(JsValue::from_str("\"0x1234567\"")),
        "net_version" => Ok(JsValue::from_str("\"1\"")),
        _ => Ok(JsValue::NULL),
    }
}

#[wasm_bindgen(js_name = "op_sign_message")]
pub async fn sign_message(message: String) -> Result<JsValue, JsValue> {
    console::log_1(&format!("Signing message: {}", message).into());
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"signature".into(), &"0xsignature".into())?;
    js_sys::Reflect::set(&obj, &"message".into(), &message.into())?;
    Ok(obj.into())
}

// Register all ops for this module
pub fn register_ops(ops: &js_sys::Object) -> Result<(), JsValue> {
    use js_sys::Reflect;
    
    Reflect::set(ops, &"op_require_payment".into(), &require_payment.into())?;
    Reflect::set(ops, &"op_sign_message".into(), &sign_message.into())?;
    Reflect::set(ops, &"op_send_async".into(), &send_async.into())?;
    Reflect::set(ops, &"op_convert_message_to_object".into(), &convert_message_to_object.into())?;
    
    Ok(())
}
