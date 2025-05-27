use sandbox::Sandbox;
use scene_executor::run_scene;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::DedicatedWorkerGlobalScope;
use js_sys::SharedArrayBuffer;
use wasm_bindgen_futures::spawn_local;

mod sandbox;
mod test_runtime;
mod sdk_runtime;
mod scene_executor;

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
    
    // Define your scene code here (for testing)
    let scene_code = test_runtime::TestRuntime::get_test_sandbox_security();
    
    // Spawn the async task and forget about it
    spawn_local(async move {
        if let Err(e) = run_scene(scene_code, true, true, true).await {
            web_sys::console::error_1(&format!("Scene execution error: {:?}", e).into());
        }
    });
    
    Ok(())
}
