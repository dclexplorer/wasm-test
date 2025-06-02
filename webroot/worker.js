// This will be Instance B of our Wasm module
let read_shared;

self.onmessage = async (event) => {
    const { type } = event.data;
    if (type === 'INIT_WASM') {
        const { wasmBytes, memory } = event.data;
        console.log("[Worker JS] Received INIT_WASM message.");
        if (!wasmBytes || !memory) {
            console.error("[Worker JS] Wasm bytes or memory not received.");
            self.postMessage("Error: Wasm bytes or memory not provided to worker.");
            return;
        }
        try {
            console.log("[Worker JS] Importing wasm-bindgen glue code...");
            // Debug: log worker location info
            const workerUrl = self.location.origin + '/pkg/worker_crate.js'
            
            // Try a direct import without URL construction
            const initModule = await import(workerUrl);
            
            console.log("[Worker JS] Initializing Wasm module (Instance B) in worker...");
            console.log("[Worker JS] initModule stringify", JSON.stringify(initModule));
            // Initialize the Wasm module with the received bytes and SHARED memory
            await initModule.default({wasmBytes, memory});
            console.log("[Worker JS] Wasm module (Instance B) initialized in worker.");

            await initModule.start()

            // Assign the specific Rust functions we need from the initialized module
            //read_shared = initModule.read_shared_from_worker;


            self.postMessage("Worker Wasm (Instance B) initialized successfully.");
        } catch (error) {
            console.error("[Worker JS] Error initializing Wasm in worker:", error);
            self.postMessage(`Error initializing Wasm in worker: ${error.message} \nStack: ${error.stack}`);
        }
    } else if (type === 'READ_STRUCT') {
        console.log("[Worker JS] Received READ_STRUCT message.");
        try {
            const result = read_shared();
            console.log("[Worker JS] Rust function executed. Result:", result);
            self.postMessage(result);
        } catch (error) {
            console.error("[Worker JS] Error calling Rust function from worker:", error);
            self.postMessage(`Error in worker Rust call: ${error}`);
        }
    }
};

console.log("[Worker JS] Script loaded.");