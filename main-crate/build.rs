// Copies the auto-generated worker bootstrap JS into OUT_DIR so lib.rs
// can embed it with `include_str!`.
fn main() {
    let js_glue = std::fs::read_to_string("../webroot/pkg/worker_crate.js")
        .expect("scene_runner.js not built yet; run wasm-pack first");
    
    // Read the WASM file and convert to base64
    let wasm_bytes = std::fs::read("../webroot/pkg/worker_crate_bg.wasm")
        .expect("scene_runner_bg.wasm not built yet; run wasm-pack first");
    let wasm_base64 = base64::encode(&wasm_bytes);
    
    // Create a self-contained worker bootstrap that includes everything
    let worker_boot = format!(
        r#"
// Inline the scene_runner.js content
{}

// Decode and instantiate the WASM module from base64
(async function() {{
    const wasmBase64 = "{}";
    const wasmBytes = Uint8Array.from(atob(wasmBase64), c => c.charCodeAt(0));
    
    try {{
        // For no-modules target, wasm_bindgen is the entry point
        await wasm_bindgen(wasmBytes);
        console.log('[WORKER JS] WASM module initialized successfully');
        // The start function should be called automatically by wasm-bindgen
    }} catch (err) {{
        console.error('[WORKER JS] Failed to initialize WASM module:', err);
    }}
}})();
"#,
        js_glue,
        wasm_base64
    );
    
    let out = std::env::var("OUT_DIR").unwrap();
    std::fs::write(format!("{out}/worker_boot.js"), worker_boot)
        .expect("failed to write worker_boot.js");
    
    println!("cargo:rerun-if-changed=../webroot/pkg/worker_crate.js");
    println!("cargo:rerun-if-changed=../webroot/pkg/worker_crate_bg.wasm");
}
