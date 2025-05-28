use wasm_bindgen::prelude::*;
use js_sys::{Object, Promise};
use wasm_bindgen_futures::future_to_promise;
use web_sys::console;

pub fn register_ops(ops: &Object) -> Result<(), JsValue> {
    // op_move_player_to - accepts 11 parameters, we'll use rest parameters
    let move_player = Closure::wrap(Box::new(
        |x: f64, y: f64, z: f64, has_camera: bool, cam_x: f64, cam_y: f64, cam_z: f64| {
            // Note: We're dropping the last 4 parameters due to Closure limitation
            // In practice, you might want to pass an object instead
            console::log_1(&format!(
                "op_move_player_to called: pos({},{},{}), camera({},{},{},{})",
                x, y, z, has_camera, cam_x, cam_y, cam_z
            ).into());
        }
    ) as Box<dyn Fn(f64, f64, f64, bool, f64, f64, f64)>);
    
    // Create a wrapper function that accepts all 11 parameters
    let move_player_wrapper = js_sys::Function::new_with_args(
        "x,y,z,has_camera,cam_x,cam_y,cam_z,has_avatar,av_x,av_y,av_z",
        "console.log('op_move_player_to called with all 11 params'); \
         console.log('pos:', x, y, z); \
         console.log('camera:', has_camera, cam_x, cam_y, cam_z); \
         console.log('avatar:', has_avatar, av_x, av_y, av_z);"
    );
    
    super::ops::register_op(ops, "op_move_player_to", &move_player_wrapper)?;
    move_player.forget();
    
    // op_emote
    let emote = Closure::wrap(Box::new(|emote_id: String| {
        console::log_1(&format!("op_emote called with: {}", emote_id).into());
    }) as Box<dyn Fn(String)>);
    
    super::ops::register_op(ops, "op_emote", emote.as_ref())?;
    emote.forget();
    
    // op_scene_emote
    let scene_emote = Closure::wrap(Box::new(|src: String, looping: bool| {
        console::log_1(&format!("op_scene_emote called with src: {}, looping: {}", src, looping).into());
    }) as Box<dyn Fn(String, bool)>);
    
    super::ops::register_op(ops, "op_scene_emote", scene_emote.as_ref())?;
    scene_emote.forget();
    
    // op_change_realm
    let change_realm = Closure::wrap(Box::new(|realm: String, message: Option<String>| -> Promise {
        console::log_1(&format!("op_change_realm called with realm: {}, message: {:?}", realm, message).into());
        
        future_to_promise(async move {
            Ok(JsValue::from_bool(true))
        })
    }) as Box<dyn Fn(String, Option<String>) -> Promise>);
    
    super::ops::register_op(ops, "op_change_realm", change_realm.as_ref())?;
    change_realm.forget();
    
    // op_external_url
    let external_url = Closure::wrap(Box::new(|url: String| -> Promise {
        console::log_1(&format!("op_external_url called with url: {}", url).into());
        
        future_to_promise(async move {
            Ok(JsValue::from_bool(true))
        })
    }) as Box<dyn Fn(String) -> Promise>);
    
    super::ops::register_op(ops, "op_external_url", external_url.as_ref())?;
    external_url.forget();
    
    // op_open_nft_dialog
    let open_nft_dialog = Closure::wrap(Box::new(|urn: String| -> Promise {
        console::log_1(&format!("op_open_nft_dialog called with urn: {}", urn).into());
        
        future_to_promise(async move {
            Ok(JsValue::from_bool(true))
        })
    }) as Box<dyn Fn(String) -> Promise>);
    
    super::ops::register_op(ops, "op_open_nft_dialog", open_nft_dialog.as_ref())?;
    open_nft_dialog.forget();
    
    // op_set_ui_focus
    let set_ui_focus = Closure::wrap(Box::new(|element_id: String| -> Promise {
        console::log_1(&format!("op_set_ui_focus called with element_id: {}", element_id).into());
        
        future_to_promise(async move {
            Ok(JsValue::UNDEFINED)
        })
    }) as Box<dyn Fn(String) -> Promise>);
    
    super::ops::register_op(ops, "op_set_ui_focus", set_ui_focus.as_ref())?;
    set_ui_focus.forget();
    
    Ok(())
}
