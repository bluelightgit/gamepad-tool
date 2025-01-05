// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use util::gamepad_util::{record_polling_rate, GamepadInfo, GamepadState};
mod util {
    pub mod gamepad_util;
}
fn main() {
    // gamepad_tool_lib::run()
    let mutex_state = Mutex::new(GamepadState::new());
    record_polling_rate(0, &mutex_state);
    let mut gamepad_state = mutex_state.lock().unwrap();
    let output = gamepad_state.get_polling_rate(0);
    println!("{:#?}", output);
}
