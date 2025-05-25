use wasm_bindgen::prelude::*;
use js_sys::{SharedArrayBuffer};

const BUFFER_SIZE: usize = 256;

pub struct SharedChannel {
    buffer: SharedArrayBuffer,
}

impl SharedChannel {
    pub fn new() -> Result<Self, JsValue> {
        let total_size = BUFFER_SIZE;
        let buffer = SharedArrayBuffer::new(total_size as u32);
        
        Ok(Self { buffer })
    }
    
    pub fn buffer(&self) -> &SharedArrayBuffer {
        &self.buffer
    }
}
