use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

mod worker;
use worker::WorkerHandle;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    web_sys::console::log_1(&"[MAIN] Start".into());

    wasm_bindgen_futures::spawn_local(async {
        if let Err(e) = WorkerHandle::spawn().await {
            web_sys::console::error_1(&e);
        }
    });

    Ok(())
}
