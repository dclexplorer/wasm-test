use wasm_bindgen::prelude::*;
use js_sys::Object;
use web_sys::console;

pub fn register_ops(ops: &Object) -> Result<(), JsValue> {
    // op_get_user_data
    let get_user_data = Closure::wrap(Box::new(|| -> JsValue {
        console::log_1(&"op_get_user_data called".into());
        
        // Return mock user data
        let data = Object::new();
        js_sys::Reflect::set(&data, &"publicKey".into(), &"0x1234567890abcdef".into()).unwrap();
        js_sys::Reflect::set(&data, &"displayName".into(), &"Test User".into()).unwrap();
        js_sys::Reflect::set(&data, &"hasConnectedWeb3".into(), &true.into()).unwrap();
        
        data.into()
    }) as Box<dyn Fn() -> JsValue>);
    
    super::ops::register_op(ops, "op_get_user_data", get_user_data.as_ref())?;
    get_user_data.forget();
    
    Ok(())
}
