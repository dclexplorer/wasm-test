// Re-export all module functions
pub use user_identity::*;
pub use user_action::*;
pub use testing::*;
pub use system_api::*;
pub use signed_fetch::*;
pub use runtime::*;
pub use restricted_actions::*;
pub use portable_experiences::*;
pub use players::*;
pub use ethereum_controller::*;
pub use environment_api::*;
pub use engine_api::*;
pub use communications_controller::*;

pub mod user_identity;
pub mod user_action;
pub mod testing;
pub mod system_api;
pub mod signed_fetch;
pub mod runtime;
pub mod restricted_actions;
pub mod portable_experiences;
pub mod players;
pub mod ethereum_controller;
pub mod environment_api;
pub mod engine_api;
pub mod communications_controller;

// Register all ops from all modules
pub fn register_all_ops(ops: &js_sys::Object) -> Result<(), wasm_bindgen::JsValue> {
    user_identity::register_ops(ops)?;
    user_action::register_ops(ops)?;
    testing::register_ops(ops)?;
    system_api::register_ops(ops)?;
    signed_fetch::register_ops(ops)?;
    runtime::register_ops(ops)?;
    restricted_actions::register_ops(ops)?;
    portable_experiences::register_ops(ops)?;
    players::register_ops(ops)?;
    ethereum_controller::register_ops(ops)?;
    environment_api::register_ops(ops)?;
    engine_api::register_ops(ops)?;
    communications_controller::register_ops(ops)?;
    
    Ok(())
}
