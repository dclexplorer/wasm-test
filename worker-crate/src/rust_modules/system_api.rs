use wasm_bindgen::prelude::*;
use js_sys::Array;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_check_for_update)]
    pub async fn js_check_for_update() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_motd)]
    pub async fn js_motd() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_get_current_login)]
    pub fn js_get_current_login() -> JsValue;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_get_previous_login)]
    pub fn js_get_previous_login() -> JsValue;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_login_previous)]
    pub async fn js_login_previous() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_login_new_code)]
    pub async fn js_login_new_code() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_login_new_success)]
    pub async fn js_login_new_success(code: String) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_login_guest)]
    pub async fn js_login_guest() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_login_cancel)]
    pub async fn js_login_cancel() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_logout)]
    pub async fn js_logout() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_settings)]
    pub async fn js_settings() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_set_setting)]
    pub async fn js_set_setting(name: String, value: JsValue) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_console_command)]
    pub async fn js_console_command(command: String, args: Array) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_kernel_fetch_headers)]
    pub async fn js_kernel_fetch_headers(url: String, method: Option<String>, meta: JsValue) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_set_avatar)]
    pub async fn js_set_avatar(base: JsValue, equip: JsValue) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_native_input)]
    pub fn js_native_input(enabled: bool);
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_get_bindings)]
    pub async fn js_get_bindings() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_set_bindings)]
    pub async fn js_set_bindings(bindings: JsValue) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_live_scene_info)]
    pub async fn js_live_scene_info(parcel: JsValue) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_get_home_scene)]
    pub async fn js_get_home_scene() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_set_home_scene)]
    pub async fn js_set_home_scene(realm: String, parcel: Array) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_get_realm_provider)]
    pub async fn js_get_realm_provider(realm_name: String) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_get_system_action_stream)]
    pub async fn js_get_system_action_stream() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_read_system_action_stream)]
    pub async fn js_read_system_action_stream(stream_id: f64) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_get_chat_stream)]
    pub async fn js_get_chat_stream() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_read_chat_stream)]
    pub async fn js_read_chat_stream(stream_id: f64) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_send_chat)]
    pub async fn js_send_chat(message: String) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen(js_name = "op_check_for_update")]
pub async fn check_for_update() -> Result<JsValue, JsValue> {
    console::log_1(&"op_check_for_update called".into());
    
    let result = Array::new();
    result.push(&JsValue::NULL); // no description
    result.push(&JsValue::NULL); // no url
    Ok(result.into())
}

#[wasm_bindgen(js_name = "op_motd")]
pub async fn motd() -> Result<JsValue, JsValue> {
    console::log_1(&"op_motd called".into());
    Ok(JsValue::from_str("Welcome to Decentraland!"))
}

#[wasm_bindgen(js_name = "op_get_current_login")]
pub fn get_current_login() -> JsValue {
    console::log_1(&"op_get_current_login called".into());
    JsValue::from_str("guest-user")
}

#[wasm_bindgen(js_name = "op_get_previous_login")]
pub fn get_previous_login() -> JsValue {
    console::log_1(&"op_get_previous_login called".into());
    JsValue::NULL
}

#[wasm_bindgen(js_name = "op_login_previous")]
pub async fn login_previous() -> Result<JsValue, JsValue> {
    console::log_1(&"op_login_previous called".into());
    Ok(JsValue::from_bool(true))
}

#[wasm_bindgen(js_name = "op_login_new_code")]
pub async fn login_new_code() -> Result<JsValue, JsValue> {
    console::log_1(&"op_login_new_code called".into());
    Ok(JsValue::from_str("login-code-123"))
}

#[wasm_bindgen(js_name = "op_login_new_success")]
pub async fn login_new_success(code: String) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_login_new_success called with code: {}", code).into());
    Ok(JsValue::from_bool(true))
}

#[wasm_bindgen(js_name = "op_login_guest")]
pub async fn login_guest() -> Result<JsValue, JsValue> {
    console::log_1(&"op_login_guest called".into());
    Ok(JsValue::from_bool(true))
}

#[wasm_bindgen(js_name = "op_login_cancel")]
pub async fn login_cancel() -> Result<JsValue, JsValue> {
    console::log_1(&"op_login_cancel called".into());
    Ok(JsValue::UNDEFINED)
}

#[wasm_bindgen(js_name = "op_logout")]
pub async fn logout() -> Result<JsValue, JsValue> {
    console::log_1(&"op_logout called".into());
    Ok(JsValue::UNDEFINED)
}

#[wasm_bindgen(js_name = "op_settings")]
pub async fn settings() -> Result<JsValue, JsValue> {
    console::log_1(&"op_settings called".into());
    let settings_array = Array::new();
    Ok(settings_array.into())
}

#[wasm_bindgen(js_name = "op_set_setting")]
pub async fn set_setting(name: String, value: JsValue) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_set_setting called with name: {}, value: {:?}", name, value).into());
    Ok(JsValue::UNDEFINED)
}

#[wasm_bindgen(js_name = "op_console_command")]
pub async fn console_command(command: String, args: Array) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_console_command called with command: {}, args: {:?}", command, args).into());
    Ok(JsValue::from_str("Command executed"))
}

#[wasm_bindgen(js_name = "op_kernel_fetch_headers")]
pub async fn kernel_fetch_headers(url: String, _method: Option<String>, _meta: JsValue) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_kernel_fetch_headers called for URL: {}", url).into());
    Ok(JsValue::NULL)
}

#[wasm_bindgen(js_name = "op_set_avatar")]
pub async fn set_avatar(_base: JsValue, _equip: JsValue) -> Result<JsValue, JsValue> {
    console::log_1(&"op_set_avatar called".into());
    Ok(JsValue::UNDEFINED)
}

#[wasm_bindgen(js_name = "op_native_input")]
pub fn native_input(enabled: bool) {
    console::log_1(&format!("op_native_input called with enabled: {}", enabled).into());
}

#[wasm_bindgen(js_name = "op_get_bindings")]
pub async fn get_bindings() -> Result<JsValue, JsValue> {
    console::log_1(&"op_get_bindings called".into());
    let bindings = Array::new();
    Ok(bindings.into())
}

#[wasm_bindgen(js_name = "op_set_bindings")]
pub async fn set_bindings(_bindings: JsValue) -> Result<JsValue, JsValue> {
    console::log_1(&"op_set_bindings called".into());
    Ok(JsValue::UNDEFINED)
}

#[wasm_bindgen(js_name = "op_live_scene_info")]
pub async fn live_scene_info(parcel: JsValue) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_live_scene_info called with parcel: {:?}", parcel).into());
    Ok(JsValue::NULL)
}

#[wasm_bindgen(js_name = "op_get_home_scene")]
pub async fn get_home_scene() -> Result<JsValue, JsValue> {
    console::log_1(&"op_get_home_scene called".into());
    let result = Array::new();
    result.push(&JsValue::from_str("default-realm"));
    result.push(&Array::new().into());
    Ok(result.into())
}

#[wasm_bindgen(js_name = "op_set_home_scene")]
pub async fn set_home_scene(realm: String, _parcel: Array) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_set_home_scene called with realm: {}", realm).into());
    Ok(JsValue::UNDEFINED)
}

#[wasm_bindgen(js_name = "op_get_realm_provider")]
pub async fn get_realm_provider(realm_name: String) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_get_realm_provider called with realm: {}", realm_name).into());
    Ok(JsValue::from_str("https://peer.decentraland.org"))
}

#[wasm_bindgen(js_name = "op_get_system_action_stream")]
pub async fn get_system_action_stream() -> Result<JsValue, JsValue> {
    console::log_1(&"op_get_system_action_stream called".into());
    Ok(JsValue::from_f64(1.0)) // Mock stream ID
}

#[wasm_bindgen(js_name = "op_read_system_action_stream")]
pub async fn read_system_action_stream(stream_id: f64) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_read_system_action_stream called with stream_id: {}", stream_id).into());
    Ok(JsValue::NULL)
}

#[wasm_bindgen(js_name = "op_get_chat_stream")]
pub async fn get_chat_stream() -> Result<JsValue, JsValue> {
    console::log_1(&"op_get_chat_stream called".into());
    Ok(JsValue::from_f64(2.0)) // Mock stream ID
}

#[wasm_bindgen(js_name = "op_read_chat_stream")]
pub async fn read_chat_stream(stream_id: f64) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_read_chat_stream called with stream_id: {}", stream_id).into());
    Ok(JsValue::NULL)
}

#[wasm_bindgen(js_name = "op_send_chat")]
pub async fn send_chat(message: String) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_send_chat called with message: {}", message).into());
    Ok(JsValue::UNDEFINED)
}
