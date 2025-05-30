use protocol_crate::SharedChannel;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{BlobPropertyBag, Blob, Worker, Url, Request, RequestInit, Response};

pub struct WorkerHandle {
    worker: Worker,
}

impl WorkerHandle {
    pub async fn spawn() -> Result<Self, JsValue> {
        web_sys::console::log_1(&"[MAIN] Creating worker...".into());
        
        // Fetch worker bootstrap from fixed path
        let opts = RequestInit::new();
        opts.set_method("GET");
        let request = Request::new_with_str_and_init("./worker.js", &opts)?;
        
        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into().unwrap();
        let boot_js = JsFuture::from(resp.text()?).await?.as_string().unwrap();
        
        let parts = js_sys::Array::of1(&JsValue::from_str(&boot_js));
        let properties = BlobPropertyBag::new();
        properties.set_type("application/javascript");
        let blob = Blob::new_with_str_sequence_and_options(&parts, &properties)?;
        let url = Url::create_object_url_with_blob(&blob)?;
        let worker = Worker::new(&url)?;
        Url::revoke_object_url(&url)?;
        
        web_sys::console::log_1(&"[MAIN] Worker created".into());

        // Fetch WASM bytes
        web_sys::console::log_1(&"[MAIN] Fetching WASM bytes...".into());
        let wasm_request = Request::new_with_str_and_init("./pkg/worker_crate_bg.wasm", &opts)?;
        let wasm_resp_value = JsFuture::from(window.fetch_with_request(&wasm_request)).await?;
        let wasm_resp: Response = wasm_resp_value.dyn_into().unwrap();
        let wasm_bytes = JsFuture::from(wasm_resp.array_buffer()?).await?;
        web_sys::console::log_1(&"[MAIN] WASM bytes fetched successfully".into());

        let shared_channel = SharedChannel::new()?;
        let shared_array_buffer = shared_channel.buffer().clone();
        
        // Delay sending both WASM bytes and SharedArrayBuffer to ensure worker is ready
        let worker_clone = worker.clone();
        let timeout = Closure::<dyn FnMut()>::new(move || {
            web_sys::console::log_1(&"[MAIN] Sending WASM bytes and SharedArrayBuffer to worker...".into());
            
            // Create message object with both WASM bytes and shared buffer
            let message = js_sys::Object::new();
            js_sys::Reflect::set(&message, &"type".into(), &"INIT_WASM".into()).unwrap();
            js_sys::Reflect::set(&message, &"wasmBytes".into(), &wasm_bytes).unwrap();
            js_sys::Reflect::set(&message, &"sharedBuffer".into(), &shared_array_buffer).unwrap();
            
            worker_clone.post_message(&message).unwrap();
            web_sys::console::log_1(&"[MAIN] WASM bytes and SharedArrayBuffer sent".into());
        });
        
        let window = web_sys::window().unwrap();
        window.set_timeout_with_callback_and_timeout_and_arguments_0(
            timeout.as_ref().unchecked_ref(),
            100  // 100ms delay to ensure worker WASM is initialized
        )?;
        timeout.forget();
        
        Ok(Self {
            worker,
        })
    }
}
