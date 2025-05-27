use wasm_bindgen::prelude::*;
use js_sys::{eval, Object, Reflect, Array, Function};

pub struct TestRuntime;

impl TestRuntime {
    /*pub fn check_v8_features(sandbox: Sandbox) -> Result<String, JsValue> {
        // For now, let's use a simpler approach - directly create and test the sandbox
        let direct_test_code = r#"
            (function(allowList, blockList) {
                const results = {
                    allowed: {},
                    blocked: {},
                    sandbox: {}
                };
                
                // Test what's in global scope first
                results.globalScope = {
                    Worker: typeof globalThis.Worker !== 'undefined',
                    WebSocket: typeof globalThis.WebSocket !== 'undefined',
                    fetch: typeof globalThis.fetch !== 'undefined'
                };
                
                // Create a test context
                const testContext = {};
                
                // Create the sandbox proxy (same as in custom_eval_sdk7)
                const proxy = new Proxy(testContext, {
                    has() {
                        return true;
                    },
                    get(target, propKey, receiver) {
                        // Track access attempts
                        if (blockList.includes(propKey)) {
                            results.blocked[propKey] = 'blocked';
                            return undefined;
                        }
                        
                        if (propKey === 'eval') return eval;
                        if (propKey === 'globalThis') return proxy;
                        if (propKey === 'global') return proxy;
                        if (propKey === 'undefined') return undefined;
                        
                        if (testContext[propKey] !== undefined) {
                            return testContext[propKey];
                        }
                        
                        if (allowList.includes(propKey)) {
                            results.allowed[propKey] = 'allowed';
                            const value = globalThis[propKey];
                            if (typeof value === 'function' && blockList.includes(value.name)) {
                                return undefined;
                            }
                            return value;
                        }
                        
                        return undefined;
                    }
                });
                
                // Test features through the proxy
                const testFeatures = [
                    'Worker', 'WebSocket', 'fetch', 'XMLHttpRequest',
                    'localStorage', 'document', 'window', 'process',
                    'Array', 'Promise', 'JSON', 'Math', 'console'
                ];
                
                for (const feature of testFeatures) {
                    try {
                        const value = proxy[feature];
                        results.sandbox[feature] = value !== undefined;
                    } catch (e) {
                        results.sandbox[feature] = false;
                    }
                }
                
                return JSON.stringify(results, null, 2);
            })
        "#;
        
        // Convert lists to JavaScript arrays
        let js_allow_list = Array::new();
        for item in &self.get_allow_list() {
            js_allow_list.push(&JsValue::from_str(item));
        }
        
        let js_block_list = Array::new();
        for item in &self.get_block_list() {
            js_block_list.push(&JsValue::from_str(item));
        }
        
        // Execute the direct test
        let test_func = eval(direct_test_code)?;
        let func = Function::from(test_func);
        
        let args = Array::new();
        args.push(&js_allow_list.into());
        args.push(&js_block_list.into());
        
        let result = func.apply(&JsValue::NULL, &args)?;
        
        result.as_string()
            .ok_or_else(|| JsValue::from_str("Failed to convert result to string"))
    }*/

    // Add a method to test the sandbox security
    pub fn test_sandbox_security(sandbox: &Sandbox) -> Result<String, JsValue> {
        let test_code = r#"
            (function() {
                const tests = {};
                
                // Test blocked features
                tests.Worker = typeof Worker !== 'undefined';
                tests.localStorage = typeof localStorage !== 'undefined';
                tests.XMLHttpRequest = typeof XMLHttpRequest !== 'undefined';
                tests.document = typeof document !== 'undefined';
                tests.window = typeof window !== 'undefined';
                tests.require = typeof require !== 'undefined';
                tests.process = typeof process !== 'undefined';
                
                // Test allowed features
                tests.Array = typeof Array !== 'undefined';
                tests.Promise = typeof Promise !== 'undefined';
                tests.JSON = typeof JSON !== 'undefined';
                tests.Math = typeof Math !== 'undefined';
                
                //return JSON.stringify(tests, null, 2);
                console.log(JSON.stringify(tests, null, 2));
            })()
        "#;
        
        // Create empty context for testing
        let context = Object::new();
        console::Console::setup_console(&context)?;
        let js_val = sandbox.custom_eval_sdk7(test_code, context.into(), true)?;
        web_sys::console::log_1(&js_val);          // <-- prints the real JS value
        web_sys::console::log_1(&js_val.js_typeof());
        Ok("check".to_string())
        /*js_val
            .and_then(|result| {
                result.as_string()
                    .ok_or_else(|| JsValue::from_str("Failed to convert result to string"))
            })*/
    }
}
