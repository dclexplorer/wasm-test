use wasm_bindgen::prelude::*;
use js_sys::{Object, Reflect, Date};
use web_sys::console;

use crate::{sandbox::Sandbox, sdk_runtime::SdkRuntime};

pub struct SceneExecutor;

impl SceneExecutor {
    pub async fn run(
        scene_code: &str,
        can_use_websocket: bool,
        can_use_fetch: bool,
        is_preview: bool,
    ) -> Result<(), JsValue> {
        // Create sandbox
        let sandbox = Sandbox::new()?;
        
        // Create the context for the scene
        let runtime_execution_context = Object::new();
        
        // Setup SDK7 runtime (includes console, module/exports, require, setImmediate, fetch, WebSocket)
        SdkRuntime::setup_runtime(
            &runtime_execution_context,
            can_use_websocket,
            can_use_fetch,
            is_preview,
        )?;
        
        // Run the code of the scene
        sandbox.custom_eval_sdk7(scene_code, runtime_execution_context.clone().into(), is_preview)?;
        
        // Get the functions once
        let on_start_func = SdkRuntime::get_on_start_func(&runtime_execution_context);
        let on_update_func = SdkRuntime::get_on_update_func(&runtime_execution_context);
        let set_immediate_list = SdkRuntime::get_set_immediate_list(&runtime_execution_context);
        
        // Check if exports contain onUpdate or onStart
        let has_on_update = on_update_func.is_some();
        let has_on_start = on_start_func.is_some();
        
        if !has_on_update && !has_on_start {
            console::error_1(&JsValue::from_str(
                "🚨🚨🚨🚨🚨 Your scene does not export an onUpdate function. Documentation: https://dcl.gg/sdk/missing-onUpdate"
            ));
            return Err(JsValue::from_str("Scene must export onUpdate or onStart"));
        }
        
        // Run onStart if it exists
        SdkRuntime::run_start(on_start_func.as_ref()).await?;
        
        // Run setImmediate callbacks after onStart
        SdkRuntime::run_set_immediate(set_immediate_list.as_ref()).await?;
        
        // Start the update loop if onUpdate exists
        if has_on_update {
            // First update always uses 0.0 as delta time
            SdkRuntime::run_update(on_update_func.as_ref(), 0.0).await?;
            SdkRuntime::run_set_immediate(set_immediate_list.as_ref()).await?;
            
            // Setup update loop (30 FPS by default)
            let update_interval_ms = 1000.0 / 30.0;
            let mut last_time = Date::now();
            
            // Create update loop using setInterval
            // Note: In a real implementation, you'd want to use requestAnimationFrame or similar
            // For now, we'll just run a few iterations as an example
            let max_iterations = 10; // Run 10 frames for testing
            
            for _ in 0..max_iterations {
                let now = Date::now();
                let dt_millis = now - last_time;
                last_time = now;
                
                let dt_secs = dt_millis / 1000.0;
                
                // Run update
                SdkRuntime::run_update(on_update_func.as_ref(), dt_secs).await?;
                SdkRuntime::run_set_immediate(set_immediate_list.as_ref()).await?;
                
                // Sleep for the remaining time to maintain FPS
                let elapsed = Date::now() - now;
                let sleep_time = (update_interval_ms - elapsed).max(0.0);
                
                // In a real implementation, you'd use setTimeout or similar
                // For now, we'll just continue
                if sleep_time > 0.0 {
                    // Note: JavaScript's sleep would go here
                    // await sleep(sleep_time)
                }
            }
            
            console::log_1(&JsValue::from_str("Scene executor: Completed test run"));
        }
        
        Ok(())
    }
    
    fn check_export_exists(context: &Object, export_name: &str) -> Result<bool, JsValue> {
        if let Ok(module) = Reflect::get(context, &"module".into()) {
            if let Ok(exports) = Reflect::get(&module, &"exports".into()) {
                if let Ok(value) = Reflect::get(&exports, &export_name.into()) {
                    return Ok(!value.is_undefined() && !value.is_null());
                }
            }
        }
        Ok(false)
    }
}

// Export a simple run function for testing
pub async fn run_scene(
    scene_code: String,
    can_use_websocket: bool,
    can_use_fetch: bool,
    is_preview: bool,
) -> Result<(), JsValue> {
    SceneExecutor::run(&scene_code, can_use_websocket, can_use_fetch, is_preview).await
}
