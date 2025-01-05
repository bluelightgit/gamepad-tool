use std::{collections::HashMap, sync::{Arc, Mutex}, thread::JoinHandle};

use tauri::Listener;
use util::gamepad_util::{GamepadInfo, GamepadState};
mod util {
    pub mod gamepad_util;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .manage(
            GamepadState::new()
        )
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            app.listen("gamepads_info", |event| {
                if let Ok(payload) = serde_json::from_str::<HashMap<u32, GamepadInfo>>(&event.payload()){
                    log::info!("Received gamepad info: {:?}", payload);
                };
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
    let app_handle = app.handle().clone();
    let mutex_state = Mutex::new(GamepadState::new());
    let handle = util::gamepad_util::start_update_thread(app_handle, mutex_state);
    // let handle1 = util::gamepad_util::record_polling_rate(0, mutex_state);
    app.run(|_, _: tauri::RunEvent| {
        // println!("{:?}", e);
    });
    handle.join().unwrap();
    // handle1.join().unwrap();
}
