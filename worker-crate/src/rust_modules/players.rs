use wasm_bindgen::prelude::*;
use js_sys::Array;
use serde::{Serialize, Deserialize};
use web_sys::console;

#[derive(Serialize, Deserialize)]
pub struct PlayerData {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[wasm_bindgen(js_name = "op_get_player_data")]
pub async fn get_player_data(user_id: String) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_get_player_data called with user_id: {}", user_id).into());
    
    let player_data = PlayerData {
        user_id,
        display_name: "Test Player".to_string(),
    };
    
    Ok(serde_wasm_bindgen::to_value(&player_data)?)
}

#[wasm_bindgen(js_name = "op_get_players_in_scene")]
pub async fn get_players_in_scene() -> Result<JsValue, JsValue> {
    console::log_1(&"op_get_players_in_scene called".into());
    
    let players = Array::new();
    players.push(&"0x1234567890abcdef".into());
    Ok(players.into())
}

#[wasm_bindgen(js_name = "op_get_connected_players")]
pub async fn get_connected_players() -> Result<JsValue, JsValue> {
    console::log_1(&"op_get_connected_players called".into());
    
    let players = Array::new();
    players.push(&"0x1234567890abcdef".into());
    Ok(players.into())
}
