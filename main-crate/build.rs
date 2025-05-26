fn main() {
    let js_glue = std::fs::read_to_string("../webroot/pkg/worker_crate.js")
        .expect("scene_runner.js not built yet; run wasm-pack first");

    let wasm_bytes = std::fs::read("../webroot/pkg/worker_crate_bg.wasm")
        .expect("scene_runner_bg.wasm not built yet; run wasm-pack first");
    let wasm_base64 = base64::encode(&wasm_bytes);

    let worker_boot = format!(
        r#"
// Inline the wasm-bindgen JS glue
{}

(async function() {{
    const wasmBase64 = "{}";
    const wasmBytes = Uint8Array.from(atob(wasmBase64), c => c.charCodeAt(0));

    // Create a SharedArrayBuffer-backed WebAssembly.Memory
    const memory = new WebAssembly.Memory({{
        initial: 256,
        maximum: 512,
        shared: true
    }});

    try {{
        // Pass memory directly to wasm_bindgen
        await wasm_bindgen(wasmBytes, memory);
        console.log('[WORKER JS] WASM module initialized with shared memory');
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
