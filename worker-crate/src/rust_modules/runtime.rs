use wasm_bindgen::prelude::*;
use js_sys::{Array, Object, Promise};
use wasm_bindgen_futures::future_to_promise;
use web_sys::console;

pub fn register_ops(ops: &Object) -> Result<(), JsValue> {
    // op_realm_information
    let realm_info = Closure::wrap(Box::new(|| -> Promise {
        console::log_1(&"op_realm_information called".into());
        
        future_to_promise(async move {
            let info = Object::new();
            js_sys::Reflect::set(&info, &"realmName".into(), &"test-realm".into()).unwrap();
            js_sys::Reflect::set(&info, &"baseUrl".into(), &"https://test.decentraland.org".into()).unwrap();
            js_sys::Reflect::set(&info, &"isPreview".into(), &false.into()).unwrap();
            Ok(info.into())
        })
    }) as Box<dyn Fn() -> Promise>);
    
    super::ops::register_op(ops, "op_realm_information", realm_info.as_ref())?;
    realm_info.forget();
    
    // op_scene_information
    let scene_info = Closure::wrap(Box::new(|| -> Promise {
        console::log_1(&"op_scene_information called".into());
        
        future_to_promise(async move {
            let info = Object::new();
            js_sys::Reflect::set(&info, &"urn".into(), &"urn:decentraland:entity:test".into()).unwrap();
            js_sys::Reflect::set(&info, &"baseUrl".into(), &"https://test.decentraland.org/content".into()).unwrap();
            
            let content = Array::new();
            js_sys::Reflect::set(&info, &"content".into(), &content).unwrap();
            js_sys::Reflect::set(&info, &"metadataJson".into(), &"{}".into()).unwrap();
            
            Ok(info.into())
        })
    }) as Box<dyn Fn() -> Promise>);
    
    super::ops::register_op(ops, "op_scene_information", scene_info.as_ref())?;
    scene_info.forget();
    
    // op_read_file
    let read_file = Closure::wrap(Box::new(|filename: String| -> Promise {
        console::log_1(&format!("op_read_file called with filename: {}", filename).into());
        
        future_to_promise(async move {
            let result = Object::new();
            let content = Array::new();
            js_sys::Reflect::set(&result, &"content".into(), &content).unwrap();
            js_sys::Reflect::set(&result, &"hash".into(), &"".into()).unwrap();
            Ok(result.into())
        })
    }) as Box<dyn Fn(String) -> Promise>);
    
    super::ops::register_op(ops, "op_read_file", read_file.as_ref())?;
    read_file.forget();
    
    Ok(())
}
