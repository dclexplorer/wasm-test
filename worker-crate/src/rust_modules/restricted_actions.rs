use wasm_bindgen::prelude::*;
use web_sys::console;

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

#[wasm_bindgen(js_name = "op_external_url")]
pub async fn open_external_url(url: String) -> Result<JsValue, JsValue> {
    console::log_1(&format!("Opening external URL: {}", url).into());
    Ok(JsValue::from_bool(true))
}

#[wasm_bindgen(js_name = "op_open_nft_dialog")]
pub async fn open_nft_dialog(urn: String) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_open_nft_dialog called with URN: {}", urn).into());
    Ok(JsValue::from_bool(true))
}

// Register all ops for this module
pub fn register_ops(ops: &js_sys::Object) -> Result<(), JsValue> {
    use js_sys::Reflect;
    
    Reflect::set(ops, &"op_move_player_to".into(), &move_player_to.into())?;
    Reflect::set(ops, &"op_teleport_to".into(), &teleport_to.into())?;
    Reflect::set(ops, &"op_trigger_emote".into(), &trigger_emote.into())?;
    Reflect::set(ops, &"op_change_realm".into(), &change_realm.into())?;
    Reflect::set(ops, &"op_open_external_url".into(), &open_external_url.into())?;
    Reflect::set(ops, &"op_open_nft_dialog".into(), &open_nft_dialog.into())?;
    
    Ok(())
}
