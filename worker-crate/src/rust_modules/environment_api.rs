use wasm_bindgen::prelude::*;
use js_sys::Object;

pub fn register_ops(ops: &Object) -> Result<(), JsValue> {
    // Environment API uses ops from runtime module (op_scene_information, op_realm_information)
    // which are already registered in runtime.rs
    Ok(())
}
