use wasm_bindgen::prelude::*;
use js_sys::{eval, Object, Reflect, Array, Function};
use std::collections::HashSet;

const ALLOW_LIST_ES2020: &[&str] = &[
    "Array", "ArrayBuffer", "BigInt", "BigInt64Array", "BigUint64Array",
    "Boolean", "DataView", "Date", "decodeURI", "decodeURIComponent",
    "encodeURI", "encodeURIComponent", "Error", "escape", "eval",
    "EvalError", "Float32Array", "Float64Array", "Function", "globalThis",
    "Infinity", "Int16Array", "Int32Array", "Int8Array", "isFinite",
    "isNaN", "JSON", "Map", "Math", "NaN", "Number", "Object",
    "parseFloat", "parseInt", "Promise", "Proxy", "RangeError",
    "ReferenceError", "Reflect", "RegExp", "Set", "SharedArrayBuffer",
    "String", "Symbol", "SyntaxError", "TypeError", "Uint16Array",
    "Uint32Array", "Uint8Array", "Uint8ClampedArray", "undefined",
    "unescape", "URIError", "WeakMap", "WeakSet"
];

// Explicitly blocked features for security
const BLOCK_LIST: &[&str] = &[
    // Worker APIs
    "Worker", "SharedWorker", "ServiceWorker", "ServiceWorkerContainer",
    // Network APIs (fetch and WebSocket handled separately)
    "XMLHttpRequest", "EventSource", 
    // Storage APIs
    "localStorage", "sessionStorage", "indexedDB", "webkitIndexedDB", "mozIndexedDB",
    // File System APIs
    "FileReader", "FileWriter", "FileSystem", "FileSystemSync",
    // Navigation/Window APIs
    "window", "location", "history", "navigator", "screen",
    // Import/Module APIs
    "require", "module", "exports", "import",
    // DOM APIs
    "document", "Document", "HTMLDocument", "XMLDocument",
    // Dangerous global functions
    "open", "close", "alert", "confirm", "prompt",
    // Process/System APIs (Node.js)
    "process", "Buffer", "global",
    // Timing APIs that could be abused
    "setImmediate", "clearImmediate",
    // Other potentially dangerous APIs
    "crypto", "Crypto", "SubtleCrypto", "CryptoKey",
    "performance", "Performance"
];

pub struct Sandbox {
    global: Object,
    allow_list: HashSet<String>,
    block_list: HashSet<String>,
}

impl Sandbox {
    pub fn new() -> Result<Sandbox, JsValue> {
        let global = js_sys::global();
        let mut allow_list = HashSet::new();
        for item in ALLOW_LIST_ES2020 {
            allow_list.insert(item.to_string());
        }
        
        let mut block_list = HashSet::new();
        for item in BLOCK_LIST {
            block_list.insert(item.to_string());
        }
        
        Ok(Sandbox {
            global: global.unchecked_into(),
            allow_list,
            block_list,
        })
    }

    pub fn custom_eval_sdk7(&self, code: &str, context: JsValue, preview_mode: bool) -> Result<JsValue, JsValue> {
        // Convert allow_list to JavaScript array
        let js_allow_list = Array::new();
        for item in &self.allow_list {
            js_allow_list.push(&JsValue::from_str(item));
        }
        
        // Convert block_list to JavaScript array
        let js_block_list = Array::new();
        for item in &self.block_list {
            js_block_list.push(&JsValue::from_str(item));
        }
        
        // Create the sandboxed evaluation function
        let sandbox_code = r#"
            (function(code, context, allowList, blockList, previewMode) {
                const proxy = new Proxy(context, {
                    has() {
                        return true;
                    },
                    get(target, propKey, receiver) {
                        // First check if it's explicitly blocked
                        if (blockList.includes(propKey)) {
                            return undefined;
                        }
                        
                        if (propKey === 'eval') return eval;
                        if (propKey === 'globalThis') return proxy;
                        if (propKey === 'global') return proxy;
                        if (propKey === 'undefined') return undefined;
                        
                        // Check context first
                        if (context[propKey] !== undefined) return context[propKey];
                        
                        // Only allow explicitly allowed global features
                        if (allowList.includes(propKey)) {
                            const value = globalThis[propKey];
                            // Additional check to prevent access to blocked features through allowed objects
                            if (typeof value === 'function' && blockList.includes(value.name)) {
                                return undefined;
                            }
                            return value;
                        }
                        
                        return undefined;
                    },
                    set(target, propKey, value) {
                        // Prevent setting blocked properties
                        if (blockList.includes(propKey)) {
                            return false;
                        }
                        target[propKey] = value;
                        return true;
                    }
                });
                
                const func = new Function('globalThis', 
                    previewMode 
                        ? `with (globalThis) {eval(${JSON.stringify(code)})}` 
                        : `with (globalThis) {${code}}`
                );

                console.log('Executing sandboxed code with context:', proxy);
                const result = func.call(proxy, proxy);
                return result;
                // Execute with deferred execution using Promise
                /*return new Promise((resolve, reject) => {
                    Promise.resolve().then(() => {
                        try {
                            
                            resolve(result);
                        } catch (e) {
                            reject(e);
                        }
                    });
                });*/
            })
        "#;
        
        let sandbox_func = eval(sandbox_code)?;
        let func = Function::from(sandbox_func);
        
        // Create arguments array
        let args = Array::new();
        args.push(&JsValue::from_str(code));
        args.push(&context);
        args.push(&js_allow_list.into());
        args.push(&js_block_list.into());
        args.push(&JsValue::from_bool(preview_mode));
        
        func.apply(&JsValue::NULL, &args)
    }

    // Getters for testing purposes - return cloned values for wasm_bindgen compatibility
    pub fn get_allow_list(&self) -> Vec<String> {
        self.allow_list.iter().cloned().collect()
    }

    pub fn get_block_list(&self) -> Vec<String> {
        self.block_list.iter().cloned().collect()
    }
    
    // Get reference to global object for internal use
    pub(crate) fn get_global(&self) -> &Object {
        &self.global
    }
}