// // Re-export all module functions
// pub use user_identity::*;
// pub use user_action::*;
// pub use testing::*;
// pub use system_api::*;
// pub use signed_fetch::*;
// pub use runtime::*;
// pub use restricted_actions::*;
// pub use portable_experiences::*;
// pub use players::*;
// pub use ethereum_controller::*;
// pub use environment_api::*;
// pub use engine_api::*;
// pub use communications_controller::*;

use wasm_bindgen::JsValue;

pub mod export_global;
// pub mod user_identity;
// pub mod user_action;
pub mod testing;
// pub mod system_api;
// pub mod signed_fetch;
// pub mod runtime;
// pub mod restricted_actions;
// pub mod portable_experiences;
// pub mod players;
// pub mod ethereum_controller;
// pub mod environment_api;
// pub mod engine_api;
// pub mod communications_controller;


pub fn register_all_ops(object: &js_sys::Object) -> Result<(), JsValue> {
    testing::register_ops(object)?;
    Ok(())
}
