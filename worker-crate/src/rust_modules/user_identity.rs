use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use js_sys::{Object, Reflect};

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export interface UserData {
    publicKey: string;
    displayName: string;
    hasConnectedWeb3: boolean;
}
"#;

#[derive(Serialize, Deserialize)]
pub struct UserData {
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "hasConnectedWeb3")]
    pub has_connected_web3: bool,
}

#[wasm_bindgen(js_name = "op_get_user_data")]
pub async fn get_user_data() -> Result<JsValue, JsValue> {
    console::log_1(&"op_get_user_data called".into());
    
    // Create mock user data
    let user_data = Object::new();
    Reflect::set(&user_data, &"userId".into(), &JsValue::from_str("0x1234567890"))?;
    Reflect::set(&user_data, &"displayName".into(), &JsValue::from_str("Guest User"))?;
    Reflect::set(&user_data, &"publicKey".into(), &JsValue::from_str("0xpublic"))?;
    Reflect::set(&user_data, &"hasConnectedWeb3".into(), &JsValue::from_bool(false))?;
    
    Ok(user_data.into())
}

// Register all ops for this module
pub fn register_ops(ops: &js_sys::Object) -> Result<(), JsValue> {
    Reflect::set(ops, &"op_get_user_data".into(), &get_user_data.into())?;
    
    Ok(())
}
