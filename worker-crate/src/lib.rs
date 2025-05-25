use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::DedicatedWorkerGlobalScope;
use js_sys::SharedArrayBuffer;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    web_sys::console::log_1(&"[WORKER] Main JS...".into());
    let global = js_sys::global().dyn_into::<DedicatedWorkerGlobalScope>()?;
    
    let onmsg = Closure::<dyn FnMut(_)>::new(move |evt: web_sys::MessageEvent| {
        web_sys::console::log_1(&"[WORKER] On Message".into());
        let data = evt.data();
        
        // Check if it's an object with buffer and context
        if let Ok(buffer) = data.dyn_into::<SharedArrayBuffer>() {
            web_sys::console::log_1(&"[WORKER] Shared Buffer Received".into());
            // TODO: SharedBuffer Received in the WebWorker...
        }
    });
    
    global.set_onmessage(Some(onmsg.as_ref().unchecked_ref()));
    onmsg.forget();
    
    Ok(())
}
