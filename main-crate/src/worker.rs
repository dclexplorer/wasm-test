use protocol_crate::SharedChannel;
use wasm_bindgen::prelude::*;
use web_sys::{BlobPropertyBag, Blob, Worker, Url};

pub struct WorkerHandle {
    worker: Worker,
}

impl WorkerHandle {
    pub fn spawn() -> Result<Self, JsValue> {
        web_sys::console::log_1(&"[MAIN] Creating worker...".into());
        
        // Create worker from embedded bootstrap
        let boot_js = include_str!(concat!(env!("OUT_DIR"), "/worker_boot.js"));
        let parts = js_sys::Array::of1(&JsValue::from_str(boot_js));
        let properties = BlobPropertyBag::new();
        properties.set_type("application/javascript");
        let blob = Blob::new_with_str_sequence_and_options(&parts, &properties)?;
        let url = Url::create_object_url_with_blob(&blob)?;
        let worker = Worker::new(&url)?;
        Url::revoke_object_url(&url)?;
        
        web_sys::console::log_1(&"[MAIN] Worker created".into());

        let shared_channel = SharedChannel::new()?;
        let shared_array_buffer = shared_channel.buffer().clone();
        
        // Delay sending SharedArrayBuffer to ensure worker is ready
        let worker_clone = worker.clone();
        let timeout = Closure::<dyn FnMut()>::new(move || {
            web_sys::console::log_1(&"[MAIN] Sending SharedArrayBuffer to worker...".into());
            worker_clone.post_message(&shared_array_buffer).unwrap();
            web_sys::console::log_1(&"[MAIN] SharedArrayBuffer sent".into());
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
