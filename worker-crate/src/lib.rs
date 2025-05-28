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
mod rust_modules;
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
    
    // Spawn the async task and forget about it
    spawn_local(async move {
        // Fetch the scene code from external file
        match fetch_scene_code("http://localhost:8000/test_scene_runtime.js").await {
            Ok(scene_code) => {
                web_sys::console::log_1(&"[WORKER] Scene code loaded successfully".into());
                if let Err(e) = run_scene(scene_code, true, true, true).await {
                    web_sys::console::error_1(&format!("Scene execution error: {:?}", e).into());
                }
            }
            Err(e) => {
                web_sys::console::error_1(&format!("Failed to fetch scene code: {:?}", e).into());
            }
        }
    });
    
    Ok(())
}

async fn fetch_scene_code(url: &str) -> Result<js_sys::JsString, JsValue> {
    // In a worker context, we use the global scope which has fetch
    let global = js_sys::global();
    let worker_global: &DedicatedWorkerGlobalScope = global.dyn_ref()
        .ok_or_else(|| JsValue::from_str("Not in a worker context"))?;
    
    // Create a request
    let request = web_sys::Request::new_with_str(url)?;
    
    // Perform the fetch using the worker's fetch method
    let response_value = wasm_bindgen_futures::JsFuture::from(
        worker_global.fetch_with_request(&request)
    ).await?;
    
    let response: web_sys::Response = response_value.dyn_into()?;
    
    if !response.ok() {
        return Err(JsValue::from_str(&format!("HTTP error: {}", response.status())));
    }
    
    // Get the text from the response
    let text_promise = response.text()?;
    let text_value = wasm_bindgen_futures::JsFuture::from(text_promise).await?;
    
    // First, check if it's a string
    if !text_value.is_string() {
        return Err(JsValue::from_str("Response is not a string"));
    }
    
    // Get the JsString without converting to Rust String
    let js_string: js_sys::JsString = text_value.dyn_into()
        .map_err(|_| JsValue::from_str("Failed to convert to JsString"))?;
    
    Ok(js_string)
}
