use wasm_bindgen::prelude::*;
use js_sys::{Array, Object, Promise, Uint8Array};
use wasm_bindgen_futures::future_to_promise;
use web_sys::console;

pub fn register_ops(ops: &Object) -> Result<(), JsValue> {
    // op_comms_send_string
    let comms_send_string = Closure::wrap(Box::new(|message: String, target: String| -> Promise {
        console::log_1(&format!("op_comms_send_string called with message: {}, target: {}", message, target).into());
        
        future_to_promise(async move {
            Ok(JsValue::UNDEFINED)
        })
    }) as Box<dyn Fn(String, String) -> Promise>);
    
    super::ops::register_op(ops, "op_comms_send_string", comms_send_string.as_ref())?;
    comms_send_string.forget();
    
    // op_comms_send_binary_single
    let comms_send_binary = Closure::wrap(Box::new(|data: Uint8Array, address: Option<String>| -> Promise {
        console::log_1(&format!("op_comms_send_binary_single called with {} bytes, address: {:?}", 
            data.length(), address).into());
        
        future_to_promise(async move {
            Ok(JsValue::UNDEFINED)
        })
    }) as Box<dyn Fn(Uint8Array, Option<String>) -> Promise>);
    
    super::ops::register_op(ops, "op_comms_send_binary_single", comms_send_binary.as_ref())?;
    comms_send_binary.forget();
    
    // op_comms_recv_binary
    let comms_recv_binary = Closure::wrap(Box::new(|| -> Promise {
        console::log_1(&"op_comms_recv_binary called".into());
        
        future_to_promise(async move {
            let result = Array::new();
            // Return empty array of binary messages
            Ok(result.into())
        })
    }) as Box<dyn Fn() -> Promise>);
    
    super::ops::register_op(ops, "op_comms_recv_binary", comms_recv_binary.as_ref())?;
    comms_recv_binary.forget();
    
    Ok(())
}
