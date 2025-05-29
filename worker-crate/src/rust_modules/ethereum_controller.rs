use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_request_ethereum_controller)]
    pub async fn js_request_ethereum_controller() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_require_payment)]
    pub async fn js_require_payment(to_hex: String, amount: f64, currency: String) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_sign_message)]
    pub async fn js_sign_message(message: String) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_send_async)]
    pub async fn js_send_async(method: String, json_params: String) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_convert_message_to_object)]
    pub async fn js_convert_message_to_object(message: String) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen(js_name = "op_request_ethereum_controller")]
pub async fn request_ethereum_controller() -> Result<JsValue, JsValue> {
    console::log_1(&"Requesting Ethereum controller".into());
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"message".into(), &"Ethereum controller granted".into())?;
    Ok(obj.into())
}

#[wasm_bindgen(js_name = "op_convert_message_to_object")]
pub async fn convert_message_to_object(message: String) -> Result<JsValue, JsValue> {
    console::log_1(&format!("Converting message: {}", message).into());
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"message".into(), &message.into())?;
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

// Add missing function
#[wasm_bindgen(js_name = "op_require_payment")]
pub async fn require_payment(to_hex: String, amount: f64, currency: String) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_require_payment called - to: {}, amount: {}, currency: {}", to_hex, amount, currency).into());
    Ok(JsValue::from_str("0xtransaction-hash"))
}
