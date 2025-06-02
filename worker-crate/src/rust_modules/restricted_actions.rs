use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_move_player_to)]
    pub fn js_move_player_to(
        x: f32, y: f32, z: f32,
        has_camera_target: bool,
        camera_x: f32, camera_y: f32, camera_z: f32,
        has_avatar_target: bool,
        avatar_x: f32, avatar_y: f32, avatar_z: f32,
    );
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_teleport_to)]
    pub async fn js_teleport_to(x: f32, y: f32) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_trigger_emote)]
    pub fn js_trigger_emote(emote: String);
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_scene_emote)]
    pub fn js_scene_emote(src: String, looping: bool);
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_change_realm)]
    pub async fn js_change_realm(realm: String, message: Option<String>) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_open_external_url)]
    pub async fn js_open_external_url(url: String) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_namespace = ["Deno", "core", "ops"], js_name = op_open_nft_dialog)]
    pub async fn js_open_nft_dialog(urn: String) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen(js_name = "op_move_player_to")]
pub fn move_player_to(
    x: f32, y: f32, z: f32,
    has_camera_target: bool,
    camera_x: f32, camera_y: f32, camera_z: f32,
    has_avatar_target: bool,
    avatar_x: f32, avatar_y: f32, avatar_z: f32,
) {
    console::log_1(&format!("Moving player to ({}, {}, {})", x, y, z).into());
}

#[wasm_bindgen(js_name = "op_emote")]
pub fn trigger_emote(emote: String) {
    console::log_1(&format!("Triggering emote: {}", emote).into());
}

#[wasm_bindgen(js_name = "op_scene_emote")]
pub fn trigger_scene_emote(src: String, looping: bool) {
    console::log_1(&format!("Triggering scene emote: {} (looping: {})", src, looping).into());
}

#[wasm_bindgen(js_name = "op_change_realm")]
pub async fn change_realm(realm: String, message: Option<String>) -> Result<JsValue, JsValue> {
    console::log_1(&format!("Changing realm to: {}", realm).into());
    Ok(JsValue::from_bool(true))
}

#[wasm_bindgen(js_name = "op_teleport_to")]
pub async fn teleport_to(x: f32, y: f32) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_teleport_to called with coordinates: ({}, {})", x, y).into());
    Ok(JsValue::from_bool(true))
}

#[wasm_bindgen(js_name = "op_open_external_url")]
pub async fn open_external_url(url: String) -> Result<JsValue, JsValue> {
    console::log_1(&format!("Opening external URL: {}", url).into());
    Ok(JsValue::from_bool(true))
}

#[wasm_bindgen(js_name = "op_open_nft_dialog")]
pub async fn open_nft_dialog(urn: String) -> Result<JsValue, JsValue> {
    console::log_1(&format!("Opening NFT dialog for: {}", urn).into());
    Ok(JsValue::from_bool(true))
}

#[wasm_bindgen(js_name = "op_set_ui_focus")]
pub async fn set_ui_focus(element_id: String) -> Result<JsValue, JsValue> {
    console::log_1(&format!("Setting UI focus to: {}", element_id).into());
    Ok(JsValue::from_bool(true))
}
