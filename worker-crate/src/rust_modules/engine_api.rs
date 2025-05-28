use wasm_bindgen::prelude::*;
use js_sys::{Array, Object, Promise, Uint8Array};
use wasm_bindgen_futures::future_to_promise;
use web_sys::console;

pub fn register_ops(ops: &Object) -> Result<(), JsValue> {
    // op_crdt_send_to_renderer
    let crdt_send = Closure::wrap(Box::new(|data: Uint8Array| {
        console::log_1(&format!("op_crdt_send_to_renderer called with {} bytes", data.length()).into());
    }) as Box<dyn Fn(Uint8Array)>);
    
    super::ops::register_op(ops, "op_crdt_send_to_renderer", crdt_send.as_ref())?;
    crdt_send.forget();
    
    // op_crdt_recv_from_renderer
    let crdt_recv = Closure::wrap(Box::new(|| -> Promise {
        console::log_1(&"op_crdt_recv_from_renderer called".into());
        
        future_to_promise(async move {
            let result = Array::new();
            // Return empty array for now
            Ok(result.into())
        })
    }) as Box<dyn Fn() -> Promise>);
    
    super::ops::register_op(ops, "op_crdt_recv_from_renderer", crdt_recv.as_ref())?;
    crdt_recv.forget();
    
    // op_subscribe
    let subscribe = Closure::wrap(Box::new(|event_id: String| {
        console::log_1(&format!("op_subscribe called with event_id: {}", event_id).into());
    }) as Box<dyn Fn(String)>);
    
    super::ops::register_op(ops, "op_subscribe", subscribe.as_ref())?;
    subscribe.forget();
    
    // op_send_batch
    let send_batch = Closure::wrap(Box::new(|| -> Array {
        console::log_1(&"op_send_batch called".into());
        Array::new() // Return empty events array
    }) as Box<dyn Fn() -> Array>);
    
    super::ops::register_op(ops, "op_send_batch", send_batch.as_ref())?;
    send_batch.forget();
    
    Ok(())
}
