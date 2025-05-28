use wasm_bindgen::prelude::*;
use js_sys::{Array, Object, Promise};
use wasm_bindgen_futures::future_to_promise;
use web_sys::console;

pub fn register_ops(ops: &Object) -> Result<(), JsValue> {
    // op_portable_spawn
    let portable_spawn = Closure::wrap(Box::new(|pid: String, ens: Option<String>| -> Promise {
        console::log_1(&format!("op_portable_spawn called with pid: {}, ens: {:?}", pid, ens).into());
        
        future_to_promise(async move {
            let result = Object::new();
            js_sys::Reflect::set(&result, &"success".into(), &true.into()).unwrap();
            Ok(result.into())
        })
    }) as Box<dyn Fn(String, Option<String>) -> Promise>);
    
    super::ops::register_op(ops, "op_portable_spawn", portable_spawn.as_ref())?;
    portable_spawn.forget();
    
    // op_portable_kill
    let portable_kill = Closure::wrap(Box::new(|pid: String| -> Promise {
        console::log_1(&format!("op_portable_kill called with pid: {}", pid).into());
        
        future_to_promise(async move {
            Ok(JsValue::from_bool(true))
        })
    }) as Box<dyn Fn(String) -> Promise>);
    
    super::ops::register_op(ops, "op_portable_kill", portable_kill.as_ref())?;
    portable_kill.forget();
    
    // op_portable_list
    let portable_list = Closure::wrap(Box::new(|| -> Promise {
        console::log_1(&"op_portable_list called".into());
        
        future_to_promise(async move {
            let list = Array::new();
            Ok(list.into())
        })
    }) as Box<dyn Fn() -> Promise>);
    
    super::ops::register_op(ops, "op_portable_list", portable_list.as_ref())?;
    portable_list.forget();
    
    Ok(())
}
