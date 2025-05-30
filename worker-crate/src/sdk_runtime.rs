use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::{Array, Function, Object, Promise, Reflect};
use web_sys::console;
use wasm_bindgen_futures::JsFuture;

use crate::rust_modules;

pub struct SdkRuntime;

impl SdkRuntime {
    pub fn setup_runtime(
        context: &Object,
        can_use_websocket: bool,
        can_use_fetch: bool,
        preview_mode: bool,
    ) -> Result<(), JsValue> {
        // Setup console
        Self::setup_console(context)?;
        
        // Setup module and exports
        Self::setup_module_exports(context)?;
        
        // Setup require function
        Self::setup_require(context)?;
        
        // Setup setImmediate
        Self::setup_set_immediate(context)?;
        
        // Setup fetch and WebSocket
        Self::setup_fetch(context, can_use_fetch)?;
        Self::setup_websocket(context, can_use_websocket)?;
        
        // Setup Deno.core.ops
        Self::setup_ops(context)?;
        
        Ok(())
    }
    
    pub fn setup_console(runtime: &Object) -> Result<(), JsValue> {
        let console_obj = Object::new();
        
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
        Reflect::set(&console_obj, &"log".into(), log_closure.as_ref())?;
        Reflect::set(&console_obj, &"info".into(), info_closure.as_ref())?;
        Reflect::set(&console_obj, &"debug".into(), debug_closure.as_ref())?;
        Reflect::set(&console_obj, &"trace".into(), trace_closure.as_ref())?;
        Reflect::set(&console_obj, &"warn".into(), warn_closure.as_ref())?;
        Reflect::set(&console_obj, &"error".into(), error_closure.as_ref())?;
        
        // Define console property on runtime
        let console_descriptor = Object::new();
        Reflect::set(&console_descriptor, &"value".into(), &console_obj)?;
        Reflect::set(&console_descriptor, &"configurable".into(), &false.into())?;
        Reflect::set(&console_descriptor, &"enumerable".into(), &true.into())?;
        Reflect::set(&console_descriptor, &"writable".into(), &false.into())?;
        
        Object::define_property(runtime, &"console".into(), &console_descriptor);
        
        // Forget closures to prevent them from being dropped
        log_closure.forget();
        info_closure.forget();
        debug_closure.forget();
        trace_closure.forget();
        warn_closure.forget();
        error_closure.forget();
        
        Ok(())
    }
    
    fn setup_module_exports(context: &Object) -> Result<(), JsValue> {
        // Create module object with exports
        let module = Object::new();
        let exports = Object::new();
        
        Reflect::set(&module, &"exports".into(), &exports)?;
        
        // Define module property
        let module_descriptor = Object::new();
        Reflect::set(&module_descriptor, &"configurable".into(), &false.into())?;
        Reflect::set(&module_descriptor, &"get".into(), &{
            let module_clone = module.clone();
            let closure = Closure::wrap(Box::new(move || {
                module_clone.clone()
            }) as Box<dyn Fn() -> Object>);
            let func = closure.as_ref().clone();
            closure.forget();
            func
        })?;
        
        Object::define_property(context, &"module".into(), &module_descriptor);
        
        // Define exports property
        let exports_descriptor = Object::new();
        Reflect::set(&exports_descriptor, &"configurable".into(), &false.into())?;
        Reflect::set(&exports_descriptor, &"get".into(), &{
            let exports_clone = exports.clone();
            let closure = Closure::wrap(Box::new(move || {
                exports_clone.clone()
            }) as Box<dyn Fn() -> Object>);
            let func = closure.as_ref().clone();
            closure.forget();
            func
        })?;
        
        Object::define_property(context, &"exports".into(), &exports_descriptor);
        
        Ok(())
    }
    
    fn setup_require(context: &Object) -> Result<(), JsValue> {
        // Create require function
        let context_clone = context.clone();
        let require_closure = Closure::wrap(Box::new(move |module_name: String| -> JsValue {
            match Self::load_module(&module_name) {
                Ok(mut module_source) => {
                    // Remove shebang if present
                    if module_source.starts_with("#!") {
                        if let Some(newline_pos) = module_source.find('\n') {
                            module_source = module_source[newline_pos + 1..].to_string();
                        }
                    }
                    
                    // Wrap the module source in a function that provides CommonJS environment
                    let head = "(function (exports, require, module, __filename, __dirname) { (function (exports, require, module, __filename, __dirname) {";
                    let foot = "\n}).call(this, exports, require, module, __filename, __dirname); })";
                    let wrapped_source = format!("{}{}{}", head, module_source, foot);
                    
                    // Evaluate the wrapped source to get the wrapper function
                    match js_sys::eval(&wrapped_source) {
                        Ok(wrapper_func) => {
                            if let Some(func) = wrapper_func.dyn_ref::<Function>() {
                                // Create module context
                                let module = Object::new();
                                let exports = Object::new();
                                Reflect::set(&module, &"exports".into(), &exports).unwrap();
                                
                                // Get require function from context
                                let require_func = Reflect::get(&context_clone, &"require".into()).unwrap();
                                
                                // Extract filename and dirname from module name
                                let filename = if module_name.starts_with('~') {
                                    module_name[1..].to_string()
                                } else {
                                    module_name.clone()
                                };
                                let dirname = if let Some(slash_pos) = filename.rfind('/') {
                                    filename[..slash_pos].to_string()
                                } else {
                                    String::from(".")
                                };
                                
                                // Call the wrapper function with CommonJS context
                                let args = Array::new();
                                args.push(&exports);
                                args.push(&require_func);
                                args.push(&module);
                                args.push(&JsValue::from_str(&filename));
                                args.push(&JsValue::from_str(&dirname));
                                
                                match func.apply(&exports, &args) {
                                    Ok(_) => {
                                        // Return module.exports
                                        Reflect::get(&module, &"exports".into()).unwrap_or(JsValue::UNDEFINED)
                                    }
                                    Err(e) => {
                                        console::error_1(&format!("Error executing module '{}': {:?}", module_name, e).into());
                                        e
                                    }
                                }
                            } else {
                                console::error_1(&format!("Module '{}' did not evaluate to a function", module_name).into());
                                JsValue::from_str("Module evaluation error")
                            }
                        }
                        Err(e) => {
                            console::error_1(&format!("Error evaluating module '{}': {:?}", module_name, e).into());
                            e
                        }
                    }
                }
                Err(err) => {
                    console::error_1(&format!("Error loading module '{}': {}", module_name, err).into());
                    JsValue::from_str(&err)
                }
            }
        }) as Box<dyn Fn(String) -> JsValue>);
        
        let require_descriptor = Object::new();
        Reflect::set(&require_descriptor, &"configurable".into(), &false.into())?;
        Reflect::set(&require_descriptor, &"value".into(), require_closure.as_ref())?;
        
        Object::define_property(context, &"require".into(), &require_descriptor);
        
        require_closure.forget();
        
        Ok(())
    }
    
    fn load_module(module_spec: &str) -> Result<String, String> {
        // Include the JavaScript modules as strings
        match module_spec {
            // User module load
            "~scene.js" => {
                // For WASM, we'll need to handle this differently since we don't have OpState
                // This would need to be injected when setting up the runtime
                Err("Scene module loading not implemented in WASM".to_string())
            }
            
            // System API (only allowed for super user scene)
            "~system/BevyExplorerApi" => {
                // For WASM, we'll need a different way to check if it's a super user scene
                // For now, we'll include it - you can add permission checks later
                Ok(include_str!("js/modules/SystemApi.js").to_owned())
            }
            
            // Core module loads
            "~system/CommunicationsController" => {
                Ok(include_str!("js/modules/CommunicationsController.js").to_owned())
            }
            "~system/CommsApi" => {
                Ok(include_str!("js/modules/CommsApi.js").to_owned())
            }
            "~system/EngineApi" => {
                Ok(include_str!("js/modules/EngineApi.js").to_owned())
            }
            "~system/EnvironmentApi" => {
                Ok(include_str!("js/modules/EnvironmentApi.js").to_owned())
            }
            "~system/EthereumController" => {
                Ok(include_str!("js/modules/EthereumController.js").to_owned())
            }
            "~system/Players" => {
                Ok(include_str!("js/modules/Players.js").to_owned())
            }
            "~system/PortableExperiences" => {
                Ok(include_str!("js/modules/PortableExperiences.js").to_owned())
            }
            "~system/RestrictedActions" => {
                Ok(include_str!("js/modules/RestrictedActions.js").to_owned())
            }
            "~system/Runtime" => {
                Ok(include_str!("js/modules/Runtime.js").to_owned())
            }
            "~system/Scene" => {
                Ok(include_str!("js/modules/Scene.js").to_owned())
            }
            "~system/SignedFetch" => {
                Ok(include_str!("js/modules/SignedFetch.js").to_owned())
            }
            "~system/Testing" => {
                Ok(include_str!("js/modules/Testing.js").to_owned())
            }
            "~system/UserActionModule" => {
                Ok(include_str!("js/modules/UserActionModule.js").to_owned())
            }
            "~system/UserIdentity" => {
                Ok(include_str!("js/modules/UserIdentity.js").to_owned())
            }
            "~system/AdaptationLayerHelper" => {
                Ok(include_str!("js/modules/AdaptationLayerHelper.js").to_owned())
            }
            
            _ => Err(format!("Invalid module request: '{}'", module_spec))
        }
    }
    
    fn setup_set_immediate(context: &Object) -> Result<(), JsValue> {
        // Create a shared array for setImmediate callbacks
        let set_immediate_list = Array::new();
        
        // Create setImmediate function
        let list_clone = set_immediate_list.clone();
        let set_immediate_closure = Closure::wrap(Box::new(move |callback: Function| {
            list_clone.push(&callback);
        }) as Box<dyn Fn(Function)>);
        
        let set_immediate_descriptor = Object::new();
        Reflect::set(&set_immediate_descriptor, &"configurable".into(), &false.into())?;
        Reflect::set(&set_immediate_descriptor, &"value".into(), set_immediate_closure.as_ref())?;
        
        Object::define_property(context, &"setImmediate".into(), &set_immediate_descriptor);
        
        // Store the list on the context for later execution
        Reflect::set(context, &"__setImmediateList".into(), &set_immediate_list)?;
        
        set_immediate_closure.forget();
        
        Ok(())
    }
    
    fn setup_fetch(context: &Object, can_use_fetch: bool) -> Result<(), JsValue> {
        if can_use_fetch {
            // Get the global fetch function
            let global = js_sys::global();
            let fetch = Reflect::get(&global, &"fetch".into())?;
            
            // For now, just forward the fetch function
            // In production, you'd want to add restrictions and logging
            let fetch_descriptor = Object::new();
            Reflect::set(&fetch_descriptor, &"configurable".into(), &false.into())?;
            Reflect::set(&fetch_descriptor, &"value".into(), &fetch)?;
            
            Object::define_property(context, &"fetch".into(), &fetch_descriptor);
        } else {
            // Create a restricted fetch that always throws
            let fetch_closure = Closure::wrap(Box::new(move |_url: JsValue, _init: JsValue| -> Result<Promise, JsValue> {
                Err(JsValue::from_str("Fetch is not allowed in this context"))
            }) as Box<dyn Fn(JsValue, JsValue) -> Result<Promise, JsValue>>);
            
            let fetch_descriptor = Object::new();
            Reflect::set(&fetch_descriptor, &"configurable".into(), &false.into())?;
            Reflect::set(&fetch_descriptor, &"value".into(), fetch_closure.as_ref())?;
            
            Object::define_property(context, &"fetch".into(), &fetch_descriptor);
            
            fetch_closure.forget();
        }
        
        Ok(())
    }
    
    fn setup_websocket(context: &Object, can_use_websocket: bool) -> Result<(), JsValue> {
        if can_use_websocket {
            // Get the global WebSocket constructor
            let global = js_sys::global();
            let websocket = Reflect::get(&global, &"WebSocket".into())?;
            
            // For now, just forward the WebSocket constructor
            // In production, you'd want to add restrictions and logging
            let websocket_descriptor = Object::new();
            Reflect::set(&websocket_descriptor, &"configurable".into(), &false.into())?;
            Reflect::set(&websocket_descriptor, &"value".into(), &websocket)?;
            
            Object::define_property(context, &"WebSocket".into(), &websocket_descriptor);
        } else {
            // Create a restricted WebSocket constructor that always throws
            let websocket_closure = Closure::wrap(Box::new(move |_url: String| -> Result<JsValue, JsValue> {
                Err(JsValue::from_str("WebSocket is not allowed in this context"))
            }) as Box<dyn Fn(String) -> Result<JsValue, JsValue>>);
            
            let websocket_descriptor = Object::new();
            Reflect::set(&websocket_descriptor, &"configurable".into(), &false.into())?;
            Reflect::set(&websocket_descriptor, &"value".into(), websocket_closure.as_ref())?;
            
            Object::define_property(context, &"WebSocket".into(), &websocket_descriptor);
            
            websocket_closure.forget();
        }
        
        Ok(())
    }
    
    fn setup_ops(context: &Object) -> Result<(), JsValue> {
        // Create Deno object structure
        let deno = Object::new();
        let core = Object::new();
        let ops = Object::new();
        
        // Set up the Deno.core.ops structure
        Reflect::set(&deno, &"core".into(), &core)?;
        Reflect::set(&core, &"ops".into(), &ops)?;
        
        // Add all ops functions
        rust_modules::register_all_ops(&ops)?;
        
        // Define Deno property on context
        let deno_descriptor = Object::new();
        Reflect::set(&deno_descriptor, &"value".into(), &deno)?;
        Reflect::set(&deno_descriptor, &"configurable".into(), &false.into())?;
        Reflect::set(&deno_descriptor, &"enumerable".into(), &true.into())?;
        Reflect::set(&deno_descriptor, &"writable".into(), &false.into())?;
        
        Object::define_property(context, &"Deno".into(), &deno_descriptor);
        
        Ok(())
    }
    
    // Get the onStart function if it exists
    pub fn get_on_start_func(context: &Object) -> Option<Function> {
        if let Ok(module) = Reflect::get(context, &"module".into()) {
            if let Ok(exports) = Reflect::get(&module, &"exports".into()) {
                if let Ok(on_start) = Reflect::get(&exports, &"onStart".into()) {
                    return on_start.dyn_ref::<Function>().cloned();
                }
            }
        }
        None
    }
    
    // Get the onUpdate function if it exists
    pub fn get_on_update_func(context: &Object) -> Option<Function> {
        if let Ok(module) = Reflect::get(context, &"module".into()) {
            if let Ok(exports) = Reflect::get(&module, &"exports".into()) {
                if let Ok(on_update) = Reflect::get(&exports, &"onUpdate".into()) {
                    return on_update.dyn_ref::<Function>().cloned();
                }
            }
        }
        None
    }
    
    // Get the setImmediate callback list
    pub fn get_set_immediate_list(context: &Object) -> Option<Array> {
        if let Ok(list) = Reflect::get(context, &"__setImmediateList".into()) {
            return list.dyn_ref::<Array>().cloned();
        }
        None
    }
    
    // Run the scene's onStart function
    pub async fn run_start(on_start_func: Option<&Function>) -> Result<(), JsValue> {
        if let Some(func) = on_start_func {
            // Call onStart()
            let result = func.call0(&JsValue::NULL)?;
            
            // Check if result is a Promise
            if let Some(promise) = result.dyn_ref::<Promise>() {
                // Await the promise
                JsFuture::from(promise.clone()).await?;
            }
        }
        
        Ok(())
    }
    
    // Run the scene's onUpdate function
    pub async fn run_update(on_update_func: Option<&Function>, delta_time: f64) -> Result<(), JsValue> {
        if let Some(func) = on_update_func {
            // Call onUpdate(deltaTime)
            let args = Array::new();
            args.push(&JsValue::from_f64(delta_time));
            
            let result = func.apply(&JsValue::NULL, &args)?;
            
            // Check if result is a Promise
            if let Some(promise) = result.dyn_ref::<Promise>() {
                // Await the promise
                if let Err(e) = JsFuture::from(promise.clone()).await {
                    console::error_1(&e);
                    // Don't return error, just log it
                }
            }
        }
        
        Ok(())
    }
    
    // Helper function to run all setImmediate callbacks
    pub async fn run_set_immediate(set_immediate_list: Option<&Array>) -> Result<(), JsValue> {
        if let Some(array) = set_immediate_list {
            let length = array.length();
            if length > 0 {
                // Execute all callbacks
                for i in 0..length {
                    if let Some(func) = array.get(i).dyn_ref::<Function>() {
                        // Execute the callback
                        let result = func.call0(&JsValue::NULL)?;
                        
                        // Check if result is a Promise
                        if let Some(promise) = result.dyn_ref::<Promise>() {
                            // Await the promise
                            if let Err(e) = JsFuture::from(promise.clone()).await {
                                console::error_1(&e);
                            }
                        }
                    }
                }
                // Clear the array
                array.set_length(0);
            }
        }
        Ok(())
    }
}
