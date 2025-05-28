use wasm_bindgen::prelude::*;
use js_sys::{Object, Promise};
use wasm_bindgen_futures::future_to_promise;
use web_sys::console;

pub fn register_ops(ops: &Object) -> Result<(), JsValue> {
    // op_send_async
    let send_async = Closure::wrap(Box::new(|method: String, json_params: String| -> Promise {
        console::log_1(&format!("op_send_async called with method: {}, params: {}", method, json_params).into());
        
        future_to_promise(async move {
            // Return mock response
            match method.as_str() {
                "eth_requestAccounts" => Ok(JsValue::from_str("[\"0x1234567890abcdef\"]")),
                "eth_blockNumber" => Ok(JsValue::from_str("\"0x1234567\"")),
                "net_version" => Ok(JsValue::from_str("\"1\"")),
                _ => Ok(JsValue::NULL),
            }
        })
    }) as Box<dyn Fn(String, String) -> Promise>);
    
    super::ops::register_op(ops, "op_send_async", send_async.as_ref())?;
    send_async.forget();
    
    Ok(())
}
