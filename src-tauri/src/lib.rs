use std::sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex};

use tauri::{Listener, Manager};
use util::gamepad_util::GamepadState;
mod util {
    pub mod gamepad_util;
    pub mod math_util;
    pub mod input_wrapper;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
    let app_handle = app.handle().clone();
    // let state = app_handle.state::<Arc<Mutex<GamepadState>>>().clone();
    let state = Arc::new(Mutex::new(GamepadState::new()));
    let mutex_state = Arc::clone(&state);
    let exit_flag = Arc::new(AtomicBool::new(false));
    let thread_exit_flag = Arc::clone(&exit_flag);
    let update_thread = util::gamepad_util::start_update_thread(app_handle, mutex_state, thread_exit_flag);
    app.run(move |_, event| {
        if let tauri::RunEvent::ExitRequested { .. } = event {
            exit_flag.store(true, Ordering::SeqCst);
        }
    });
    update_thread.join().unwrap();
}
