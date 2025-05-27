use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub struct Console;

#[wasm_bindgen]
impl Console {
    pub fn setup_console(runtime: &js_sys::Object) -> Result<(), JsValue> {
        let console_obj = js_sys::Object::new();
        
        // Create closures for each console method
        let log_closure = Closure::wrap(Box::new(move |msg: JsValue| {
            console::log_1(&msg);
        }) as Box<dyn Fn(JsValue)>);
        
        let info_closure = Closure::wrap(Box::new(move |msg: JsValue| {
            console::info_1(&msg);
        }) as Box<dyn Fn(JsValue)>);
        
        let debug_closure = Closure::wrap(Box::new(move |msg: JsValue| {
            console::debug_1(&msg);
        }) as Box<dyn Fn(JsValue)>);
        
        let trace_closure = Closure::wrap(Box::new(move |msg: JsValue| {
            console::trace_1(&msg);
        }) as Box<dyn Fn(JsValue)>);
        
        let warn_closure = Closure::wrap(Box::new(move |msg: JsValue| {
            console::warn_1(&msg);
        }) as Box<dyn Fn(JsValue)>);
        
        let error_closure = Closure::wrap(Box::new(move |msg: JsValue| {
            console::error_1(&msg);
        }) as Box<dyn Fn(JsValue)>);
        
        // Set properties on console object
        js_sys::Reflect::set(&console_obj, &"log".into(), log_closure.as_ref())?;
        js_sys::Reflect::set(&console_obj, &"info".into(), info_closure.as_ref())?;
        js_sys::Reflect::set(&console_obj, &"debug".into(), debug_closure.as_ref())?;
        js_sys::Reflect::set(&console_obj, &"trace".into(), trace_closure.as_ref())?;
        js_sys::Reflect::set(&console_obj, &"warning".into(), warn_closure.as_ref())?;
        js_sys::Reflect::set(&console_obj, &"error".into(), error_closure.as_ref())?;
        
        // Define console property on runtime
        js_sys::Object::define_property(
            runtime,
            &"console".into(),
            &js_sys::Object::from_entries(&js_sys::Array::from(&JsValue::from(vec![
                vec![JsValue::from("value"), JsValue::from(console_obj)],
                vec![JsValue::from("configurable"), JsValue::from(false)],
                vec![JsValue::from("enumerable"), JsValue::from(true)],
                vec![JsValue::from("writable"), JsValue::from(false)],
            ].into_iter().map(|v| js_sys::Array::from(&JsValue::from(v))).collect::<js_sys::Array>())))?
        );
        
        // Forget closures to prevent them from being dropped
        log_closure.forget();
        info_closure.forget();
        debug_closure.forget();
        trace_closure.forget();
        warn_closure.forget();
        error_closure.forget();
        
        Ok(())
    }
}
