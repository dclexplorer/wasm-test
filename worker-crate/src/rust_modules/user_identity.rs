use wasm_bindgen::prelude::*;
use js_sys::Object;
use web_sys::console;

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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_get_user_data)]
    pub async fn js_get_user_data() -> Result<JsValue, JsValue>;
}

#[wasm_bindgen(js_name = "op_get_user_data")]
pub async fn get_user_data() -> Result<JsValue, JsValue> {
    console::log_1(&"op_get_user_data called".into());
    
    // Create mock user data
    let user_data = Object::new();
    js_sys::Reflect::set(&user_data, &"userId".into(), &JsValue::from_str("0x1234567890"))?;
    js_sys::Reflect::set(&user_data, &"displayName".into(), &JsValue::from_str("Guest User"))?;
    js_sys::Reflect::set(&user_data, &"publicKey".into(), &JsValue::from_str("0xpublic"))?;
    js_sys::Reflect::set(&user_data, &"hasConnectedWeb3".into(), &JsValue::from_bool(false))?;
    
    Ok(user_data.into())
}
