use wasm_bindgen::prelude::*;
use js_sys::{Array, Object, Reflect};
use web_sys::console;

#[wasm_bindgen(js_name = "op_signed_fetch_headers")]
pub async fn signed_fetch_headers(url: String, method: Option<String>) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_signed_fetch_headers called with url: {}, method: {:?}", url, method).into());
    
    let headers = Array::new();
    
    // Add mock signed headers
    let header1 = Array::new();
    header1.push(&"X-Identity".into());
    header1.push(&"dcl:test-user".into());
    headers.push(&header1);
    
    Ok(headers.into())
}

#[wasm_bindgen(js_name = "op_sign_fetch")]
pub async fn sign_fetch(
    url: String,
    init: String,
    metadata: JsValue
) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_sign_fetch called - URL: {}", url).into());
    
    // Create headers object
    let headers_obj = Object::new();
    Reflect::set(&headers_obj, &"X-Signed".into(), &JsValue::from_str("true"))?;
    
    Ok(headers_obj.into())
}

// Register all ops for this module
pub fn register_ops(ops: &js_sys::Object) -> Result<(), JsValue> {
    Reflect::set(ops, &"op_sign_fetch".into(), &sign_fetch.into())?;
    
    Ok(())
}
