use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

mod worker;
use worker::WorkerHandle;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    web_sys::console::log_1(&"[MAIN] Start".into());

    let _ = WorkerHandle::spawn()?;

    Ok(())
}
