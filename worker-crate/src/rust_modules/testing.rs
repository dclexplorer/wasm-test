use wasm_bindgen::prelude::*;
use js_sys::{Array, Object, Promise};
use wasm_bindgen_futures::future_to_promise;
use web_sys::console;

pub fn register_ops(ops: &Object) -> Result<(), JsValue> {
    // op_testing_enabled
    let testing_enabled = Closure::wrap(Box::new(|| -> bool {
        console::log_1(&"op_testing_enabled called".into());
        false // Testing disabled by default
    }) as Box<dyn Fn() -> bool>);
    
    super::ops::register_op(ops, "op_testing_enabled", testing_enabled.as_ref())?;
    testing_enabled.forget();
    
    // op_take_and_compare_snapshot
    let take_and_compare_snapshot = Closure::wrap(Box::new(
        |src: String, camera_pos: Array, camera_target: Array, size: Array, methods: Object| -> Promise {
            console::log_1(&format!("op_take_and_compare_snapshot called with src: {}", src).into());
            
            future_to_promise(async move {
                let result = Object::new();
                js_sys::Reflect::set(&result, &"passed".into(), &true.into()).unwrap();
                js_sys::Reflect::set(&result, &"diffPercentage".into(), &0.0.into()).unwrap();
                Ok(result.into())
            })
        }
    ) as Box<dyn Fn(String, Array, Array, Array, Object) -> Promise>);
    
    super::ops::register_op(ops, "op_take_and_compare_snapshot", take_and_compare_snapshot.as_ref())?;
    take_and_compare_snapshot.forget();
    
    // op_log_test_result
    let log_test_result = Closure::wrap(Box::new(|result: JsValue| {
        console::log_1(&format!("op_log_test_result called with: {:?}", result).into());
    }) as Box<dyn Fn(JsValue)>);
    
    super::ops::register_op(ops, "op_log_test_result", log_test_result.as_ref())?;
    log_test_result.forget();
    
    // op_log_test_plan
    let log_test_plan = Closure::wrap(Box::new(|plan: JsValue| {
        console::log_1(&format!("op_log_test_plan called with: {:?}", plan).into());
    }) as Box<dyn Fn(JsValue)>);
    
    super::ops::register_op(ops, "op_log_test_plan", log_test_plan.as_ref())?;
    log_test_plan.forget();
    
    Ok(())
}
