use wasm_bindgen::prelude::*;
use js_sys::Array;
use web_sys::console;

#[wasm_bindgen(js_name = "op_portable_spawn")]
pub async fn portable_spawn(pid: String, ens: Option<String>) -> Result<JsValue, JsValue> {
    console::log_1(&format!("Spawning portable experience: {} (ens: {:?})", pid, ens).into());
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"status".into(), &true.into())?;
    Ok(obj.into())
}

#[wasm_bindgen(js_name = "op_portable_kill")]
pub async fn portable_kill(pid: String) -> Result<bool, JsValue> {
    console::log_1(&format!("Killing portable experience: {}", pid).into());
    Ok(true)
}

#[wasm_bindgen(js_name = "op_portable_list")]
pub async fn portable_list() -> Result<JsValue, JsValue> {
    console::log_1(&"Listing portable experiences".into());
    Ok(Array::new().into())
}

#[wasm_bindgen(js_name = "op_get_portable_experience")]
pub async fn get_portable_experience(urn: String) -> Result<JsValue, JsValue> {
    console::log_1(&format!("op_get_portable_experience called with URN: {}", urn).into());
    Ok(JsValue::NULL)
}

// Register all ops for this module
pub fn register_ops(ops: &js_sys::Object) -> Result<(), JsValue> {
    use js_sys::Reflect;
    
    Reflect::set(ops, &"op_spawn".into(), &portable_spawn.into())?;
    Reflect::set(ops, &"op_kill".into(), &portable_kill.into())?;
    Reflect::set(ops, &"op_list_portable_experiences".into(), &portable_list.into())?;
    Reflect::set(ops, &"op_get_portable_experience".into(), &get_portable_experience.into())?;
    
    Ok(())
}
