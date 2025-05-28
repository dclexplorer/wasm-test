use wasm_bindgen::prelude::*;
use web_sys::console;

// Environment API ops are implemented in runtime.rs
// This module is kept for organizational purposes

// Register all ops for this module
pub fn register_ops(ops: &js_sys::Object) -> Result<(), JsValue> {
    use js_sys::Reflect;
    
    Reflect::set(ops, &"op_get_explorer_configuration".into(), &get_explorer_configuration.into())?;
    Reflect::set(ops, &"op_is_preview_mode".into(), &is_preview_mode.into())?;
    Reflect::set(ops, &"op_is_editor".into(), &is_editor.into())?;
    Reflect::set(ops, &"op_are_unsafe_requests_allowed".into(), &are_unsafe_requests_allowed.into())?;
    Reflect::set(ops, &"op_get_platform".into(), &get_platform.into())?;
    
    Ok(())
}
