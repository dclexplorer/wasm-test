use wasm_bindgen::prelude::*;
use js_sys::{Object, Promise};
use wasm_bindgen_futures::future_to_promise;
use web_sys::console;

pub fn register_ops(ops: &Object) -> Result<(), JsValue> {
    // op_teleport_to
    let teleport_to = Closure::wrap(Box::new(|x: i32, y: i32| -> Promise {
        console::log_1(&format!("op_teleport_to called with x: {}, y: {}", x, y).into());
        
        future_to_promise(async move {
            Ok(JsValue::from_str("Teleported successfully"))
        })
    }) as Box<dyn Fn(i32, i32) -> Promise>);
    
    super::ops::register_op(ops, "op_teleport_to", teleport_to.as_ref())?;
    teleport_to.forget();
    
    Ok(())
}
