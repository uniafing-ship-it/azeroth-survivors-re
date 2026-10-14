//! Web-only touch controls: synchronize the virtual buttons before gameplay Update.
use std::collections::HashSet;
use bevy::prelude::*;
use wasm_bindgen::JsValue;

#[derive(Resource, Default)]
pub(super) struct TouchHeld(HashSet<KeyCode>);

pub(super) fn poll(mut held: ResMut<TouchHeld>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    let Some(win) = web_sys::window() else { return };
    let controls = js_sys::Reflect::get(&win, &JsValue::from_str("__azeroth_touch_keys"))
        .unwrap_or(JsValue::NULL);
    const MAPPING: &[(&str, KeyCode)] = &[
        ("KeyW", KeyCode::KeyW), ("KeyA", KeyCode::KeyA),
        ("KeyS", KeyCode::KeyS), ("KeyD", KeyCode::KeyD),
        ("Space", KeyCode::Space), ("KeyF", KeyCode::KeyF),
        ("Escape", KeyCode::Escape), ("Enter", KeyCode::Enter),
        ("Digit1", KeyCode::Digit1), ("Digit2", KeyCode::Digit2),
        ("Digit3", KeyCode::Digit3), ("KeyC", KeyCode::KeyC),
    ];
    for &(name, code) in MAPPING {
        let down = js_sys::Reflect::get(&controls, &JsValue::from_str(name))
            .ok().and_then(|v| v.as_bool()).unwrap_or(false);
        if down {
            held.0.insert(code);
            keys.press(code);
        } else if held.0.remove(&code) {
            keys.release(code);
        }
    }
}
