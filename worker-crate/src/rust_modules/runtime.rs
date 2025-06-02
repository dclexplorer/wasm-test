use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use web_sys::console;

#[derive(Serialize, Deserialize)]
pub struct RealmInfo {
    #[serde(rename = "realmName")]
    pub realm_name: String,
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    #[serde(rename = "isPreview")]
    pub is_preview: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SceneInfo {
    pub urn: String,
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    pub content: Vec<ContentItem>,
    #[serde(rename = "metadataJson")]
    pub metadata_json: String,
}

#[derive(Serialize, Deserialize)]
pub struct ContentItem {
    pub file: String,
    pub hash: String,
}

#[wasm_bindgen(js_name = "op_realm_information")]
pub async fn realm_information() -> Result<JsValue, JsValue> {
    let info = RealmInfo {
        realm_name: "test-realm".to_string(),
        base_url: "https://test-realm.decentraland.org".to_string(),
        is_preview: false,
    };
    Ok(serde_wasm_bindgen::to_value(&info)?)
}

#[wasm_bindgen(js_name = "op_scene_information")]
pub async fn scene_information() -> Result<JsValue, JsValue> {
    let info = SceneInfo {
        urn: "urn:decentraland:scene:test".to_string(),
        base_url: "https://content.decentraland.org".to_string(),
        content: vec![],
        metadata_json: "{}".to_string(),
    };
    Ok(serde_wasm_bindgen::to_value(&info)?)
}

#[wasm_bindgen(js_name = "op_read_file")]
pub async fn read_file(file_name: String) -> Result<JsValue, JsValue> {
    console::log_1(&format!("Reading file: {}", file_name).into());
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"content".into(), &js_sys::Uint8Array::new_with_length(0).into())?;
    js_sys::Reflect::set(&obj, &"hash".into(), &"".into())?;
    Ok(obj.into())
}

#[wasm_bindgen(js_name = "op_stream_create")]
pub async fn stream_create() -> Result<JsValue, JsValue> {
    console::log_1(&"op_stream_create called".into());
    Ok(JsValue::from_f64(1.0)) // Return mock stream ID
}

#[wasm_bindgen(js_name = "op_stream_read")]
pub async fn stream_read(stream_id: f64, buffer_size: u32) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_stream_read called with stream_id: {}, buffer_size: {}", stream_id, buffer_size).into());
    Ok(JsValue::NULL)
}

#[wasm_bindgen(js_name = "op_stream_write")]
pub async fn stream_write(stream_id: f64, data: Vec<u8>) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_stream_write called with stream_id: {}, data length: {}", stream_id, data.len()).into());
    Ok(JsValue::from_f64(data.len() as f64))
}

#[wasm_bindgen(js_name = "op_stream_abort")]
pub async fn stream_abort(stream_id: f64) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_stream_abort called with stream_id: {}", stream_id).into());
    Ok(JsValue::UNDEFINED)
}
