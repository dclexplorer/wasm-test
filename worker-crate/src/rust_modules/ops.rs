use wasm_bindgen::prelude::*;
use js_sys::{Object, Reflect};

pub fn register_all_ops(ops: &Object) -> Result<(), JsValue> {
    // Register ops from each module
    super::user_identity::register_ops(ops)?;
    super::user_action::register_ops(ops)?;
    super::testing::register_ops(ops)?;
    super::system_api::register_ops(ops)?;
    super::signed_fetch::register_ops(ops)?;
    super::runtime::register_ops(ops)?;
    super::restricted_actions::register_ops(ops)?;
    super::portable_experiences::register_ops(ops)?;
    super::players::register_ops(ops)?;
    super::ethereum_controller::register_ops(ops)?;
    super::environment_api::register_ops(ops)?;
    super::engine_api::register_ops(ops)?;
    super::communications_controller::register_ops(ops)?;
    
    Ok(())
}

// Helper function to register a single op
pub fn register_op(ops: &Object, name: &str, func: &JsValue) -> Result<(), JsValue> {
    Reflect::set(ops, &JsValue::from_str(name), func)?;
    Ok(())
}
