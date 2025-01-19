use std::{collections::HashMap, sync::{Arc, Mutex}, thread::JoinHandle};

use tauri::{Listener, Manager};
use util::gamepad_util::{GamepadInfo, GamepadState, PollingRateLog};
mod util {
    pub mod gamepad_util;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        // .manage(
        //     Arc::new(Mutex::new(GamepadState::new()))
        // )
        .plugin(tauri_plugin_opener::init())
        // .setup(move |app| {
        //     app.listen("polling_rate_log", |event| {
        //         if let Ok(payload) = serde_json::from_str::<HashMap<u32, Vec<PollingRateLog>>>(&event.payload()){
        //             println!("{:?}", payload);
        //         };
        //     });
        //     Ok(())
        // })
        .invoke_handler(tauri::generate_handler![
            // util::gamepad_util::start_update_thread,
            // util::gamepad_util::record_polling_rate,
            // util::gamepad_util::get_polling_rate,
            // util::gamepad_util::set_frame_rate,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
    let app_handle = app.handle().clone();
    // let state = app_handle.state::<Arc<Mutex<GamepadState>>>().clone();
    let state = Arc::new(Mutex::new(GamepadState::new()));
    let mutex_state = Arc::clone(&state);
    let update_thread = util::gamepad_util::start_update_thread(app_handle, mutex_state);
    app.run(|_, _: tauri::RunEvent| {
    });
    update_thread.join().unwrap();
}
