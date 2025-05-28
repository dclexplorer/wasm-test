use wasm_bindgen::prelude::*;
use js_sys::{Array, Object, Promise};
use wasm_bindgen_futures::future_to_promise;
use web_sys::console;

pub fn register_ops(ops: &Object) -> Result<(), JsValue> {
    // op_check_for_update
    let check_for_update = Closure::wrap(Box::new(|| -> Promise {
        console::log_1(&"op_check_for_update called".into());
        
        future_to_promise(async move {
            let result = Array::new();
            result.push(&JsValue::NULL); // no description
            result.push(&JsValue::NULL); // no url
            Ok(result.into())
        })
    }) as Box<dyn Fn() -> Promise>);
    
    super::ops::register_op(ops, "op_check_for_update", check_for_update.as_ref())?;
    check_for_update.forget();
    
    // op_motd
    let motd = Closure::wrap(Box::new(|| -> Promise {
        console::log_1(&"op_motd called".into());
        
        future_to_promise(async move {
            Ok(JsValue::from_str("Welcome to Decentraland!"))
        })
    }) as Box<dyn Fn() -> Promise>);
    
    super::ops::register_op(ops, "op_motd", motd.as_ref())?;
    motd.forget();
    
    // op_get_current_login
    let get_current_login = Closure::wrap(Box::new(|| -> JsValue {
        console::log_1(&"op_get_current_login called".into());
        JsValue::from_str("guest-user")
    }) as Box<dyn Fn() -> JsValue>);
    
    super::ops::register_op(ops, "op_get_current_login", get_current_login.as_ref())?;
    get_current_login.forget();
    
    // op_settings
    let settings = Closure::wrap(Box::new(|| -> Promise {
        console::log_1(&"op_settings called".into());
        
        future_to_promise(async move {
            let settings_array = Array::new();
            Ok(settings_array.into())
        })
    }) as Box<dyn Fn() -> Promise>);
    
    super::ops::register_op(ops, "op_settings", settings.as_ref())?;
    settings.forget();
    
    // op_set_setting
    let set_setting = Closure::wrap(Box::new(|name: String, value: JsValue| -> Promise {
        console::log_1(&format!("op_set_setting called with name: {}, value: {:?}", name, value).into());
        
        future_to_promise(async move {
            Ok(JsValue::UNDEFINED)
        })
    }) as Box<dyn Fn(String, JsValue) -> Promise>);
    
    super::ops::register_op(ops, "op_set_setting", set_setting.as_ref())?;
    set_setting.forget();
    
    // op_console_command
    let console_command = Closure::wrap(Box::new(|command: String, args: Array| -> Promise {
        console::log_1(&format!("op_console_command called with command: {}, args: {:?}", command, args).into());
        
        future_to_promise(async move {
            Ok(JsValue::from_str("Command executed"))
        })
    }) as Box<dyn Fn(String, Array) -> Promise>);
    
    super::ops::register_op(ops, "op_console_command", console_command.as_ref())?;
    console_command.forget();
    
    // Add more ops as needed...
    // op_login_guest, op_logout, op_kernel_fetch_headers, etc.
    
    // Placeholder for remaining ops
    register_placeholder_ops(ops)?;
    
    Ok(())
}

fn register_placeholder_ops(ops: &Object) -> Result<(), JsValue> {
    // Quick registration of remaining ops with placeholder implementations
    let ops_list = vec![
        "op_get_previous_login", "op_login_previous", "op_login_new_code", 
        "op_login_new_success", "op_login_cancel", "op_login_guest", "op_logout",
        "op_kernel_fetch_headers", "op_set_avatar", "op_native_input",
        "op_get_bindings", "op_set_bindings", "op_live_scene_info",
        "op_get_home_scene", "op_set_home_scene", "op_get_realm_provider",
        "op_get_system_action_stream", "op_read_system_action_stream",
        "op_get_chat_stream", "op_read_chat_stream", "op_send_chat"
    ];
    
    for op_name in ops_list {
        let name = op_name.to_string();
        let closure = Closure::wrap(Box::new(move |args: JsValue| -> Promise {
            console::log_1(&format!("{} called", name).into());
            
            future_to_promise(async move {
                Ok(JsValue::UNDEFINED)
            })
        }) as Box<dyn Fn(JsValue) -> Promise>);
        
        super::ops::register_op(ops, op_name, closure.as_ref())?;
        closure.forget();
    }
    
    Ok(())
}
