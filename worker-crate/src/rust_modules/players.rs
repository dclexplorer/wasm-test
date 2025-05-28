use wasm_bindgen::prelude::*;
use js_sys::{Array, Object, Promise};
use wasm_bindgen_futures::future_to_promise;
use web_sys::console;

pub fn register_ops(ops: &Object) -> Result<(), JsValue> {
    // op_get_player_data
    let get_player_data = Closure::wrap(Box::new(|user_id: String| -> Promise {
        console::log_1(&format!("op_get_player_data called with user_id: {}", user_id).into());
        
        future_to_promise(async move {
            let player_data = Object::new();
            js_sys::Reflect::set(&player_data, &"userId".into(), &user_id.into()).unwrap();
            js_sys::Reflect::set(&player_data, &"displayName".into(), &"Test Player".into()).unwrap();
            Ok(player_data.into())
        })
    }) as Box<dyn Fn(String) -> Promise>);
    
    super::ops::register_op(ops, "op_get_player_data", get_player_data.as_ref())?;
    get_player_data.forget();
    
    // op_get_players_in_scene
    let get_players_in_scene = Closure::wrap(Box::new(|| -> Promise {
        console::log_1(&"op_get_players_in_scene called".into());
        
        future_to_promise(async move {
            let players = Array::new();
            players.push(&"0x1234567890abcdef".into());
            Ok(players.into())
        })
    }) as Box<dyn Fn() -> Promise>);
    
    super::ops::register_op(ops, "op_get_players_in_scene", get_players_in_scene.as_ref())?;
    get_players_in_scene.forget();
    
    // op_get_connected_players
    let get_connected_players = Closure::wrap(Box::new(|| -> Promise {
        console::log_1(&"op_get_connected_players called".into());
        
        future_to_promise(async move {
            let players = Array::new();
            players.push(&"0x1234567890abcdef".into());
            Ok(players.into())
        })
    }) as Box<dyn Fn() -> Promise>);
    
    super::ops::register_op(ops, "op_get_connected_players", get_connected_players.as_ref())?;
    get_connected_players.forget();
    
    Ok(())
}
