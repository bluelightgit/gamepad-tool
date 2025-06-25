use std::sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex};
use cmds::GlobalGamepadState;
use tauri::Manager;
use util::gamepad_util::GamepadState;
mod util {
    pub mod gamepad_util;
    pub mod math_util;
    pub mod input_wrapper;
}
mod cmds;
mod websocket;
mod models;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(
            GlobalGamepadState::default()
        )
        .invoke_handler(tauri::generate_handler![
            cmds::start_update,
            cmds::stop_update,
            cmds::get_gamepad_ids,
            cmds::set_log_size,
            websocket::get_websocket_port,
            websocket::start_websocket_server_command,
        ])        .setup(|app| {
            let app_handle = app.handle().clone();
            // 在应用启动时启动WebSocket服务器
            tauri::async_runtime::spawn(async move {
                websocket::start_websocket_server_command(app_handle);
            });
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
    app.run(move |app_handle, event| {
        if let tauri::RunEvent::ExitRequested { .. } = event {
            // 停止更新任务
            let state = app_handle.state::<cmds::GlobalGamepadState>();
            state.update_running.store(false, Ordering::SeqCst);
        }
    });
}
