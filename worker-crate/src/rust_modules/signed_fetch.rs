use wasm_bindgen::prelude::*;
use js_sys::{Array, Object};
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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_signed_fetch_headers)]
    pub async fn js_signed_fetch_headers(url: String, method: Option<String>) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_sign_fetch)]
    pub async fn js_sign_fetch(
        url: String,
        init: String,
        metadata: JsValue
    ) -> Result<JsValue, JsValue>;
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
    js_sys::Reflect::set(&headers_obj, &"X-Signed".into(), &JsValue::from_str("true"))?;
    
    Ok(headers_obj.into())
}
