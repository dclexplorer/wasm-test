use wasm_bindgen::prelude::*;
use js_sys::Object;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_get_explorer_configuration)]
    pub async fn js_get_explorer_configuration() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_is_preview_mode)]
    pub fn js_is_preview_mode() -> bool;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_is_editor)]
    pub fn js_is_editor() -> bool;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_are_unsafe_requests_allowed)]
    pub fn js_are_unsafe_requests_allowed() -> bool;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_get_platform)]
    pub fn js_get_platform() -> String;
}

// Add the implementation functions
#[wasm_bindgen(js_name = "op_get_explorer_configuration")]
pub async fn get_explorer_configuration() -> Result<JsValue, JsValue> {
    console::log_1(&"op_get_explorer_configuration called".into());
    
    let config = Object::new();
    js_sys::Reflect::set(&config, &"clientUri".into(), &JsValue::from_str("https://play.decentraland.org"))?;
    js_sys::Reflect::set(&config, &"configurations".into(), &Object::new())?;
    
    Ok(config.into())
}

#[wasm_bindgen(js_name = "op_is_preview_mode")]
pub fn is_preview_mode() -> bool {
    console::log_1(&"op_is_preview_mode called".into());
    false
}

#[wasm_bindgen(js_name = "op_is_editor")]
pub fn is_editor() -> bool {
    console::log_1(&"op_is_editor called".into());
    false
}

#[wasm_bindgen(js_name = "op_are_unsafe_requests_allowed")]
pub fn are_unsafe_requests_allowed() -> bool {
    console::log_1(&"op_are_unsafe_requests_allowed called".into());
    false
}

#[wasm_bindgen(js_name = "op_get_platform")]
pub fn get_platform() -> String {
    console::log_1(&"op_get_platform called".into());
    "browser".to_string()
}

// Environment API ops are implemented in runtime.rs
// This module is kept for organizational purposes

pub fn register_ops(ops: &js_sys::Object) -> Result<(), JsValue> {
    // Environment API uses ops from runtime module (op_scene_information, op_realm_information)
    // which are already registered in runtime.rs
    Ok(())
}
