use wasm_bindgen::prelude::*;
use js_sys::{Array, Promise};
use wasm_bindgen_futures::future_to_promise;
use web_sys::console;

pub fn register_ops(ops: &js_sys::Object) -> Result<(), JsValue> {
    // op_signed_fetch_headers
    let signed_fetch_headers = Closure::wrap(Box::new(|url: String, method: Option<String>| -> Promise {
        console::log_1(&format!("op_signed_fetch_headers called with url: {}, method: {:?}", url, method).into());
        
        future_to_promise(async move {
            let headers = Array::new();
            // Add mock signed headers
            let header1 = Array::new();
            header1.push(&"X-Identity".into());
            header1.push(&"dcl:test-user".into());
            headers.push(&header1);
            
            Ok(headers.into())
        })
    }) as Box<dyn Fn(String, Option<String>) -> Promise>);
    
    super::ops::register_op(ops, "op_signed_fetch_headers", signed_fetch_headers.as_ref())?;
    signed_fetch_headers.forget();
    
    Ok(())
}
