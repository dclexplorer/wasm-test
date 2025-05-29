#[macro_export]
macro_rules! export_globals {
    ($($name:ident),* $(,)?) => {
        // For functions already marked with #[wasm_bindgen], 
        // they are automatically available in the module exports.
        // This macro is now just a placeholder for organization.
        pub fn expose_all() -> Result<(), JsValue> {
            // Functions with #[wasm_bindgen] are automatically exported
            Ok(())
        }
    };
}