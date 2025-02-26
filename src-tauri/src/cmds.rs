use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use tauri::{AppHandle, Emitter};

use crate::GamepadState;

pub struct GlobalGamepadState {
    pub mutex_state: Arc<Mutex<GamepadState>>,
    /// 当值为 true 时，表示更新任务正在运行
    pub update_running: Arc<AtomicBool>,
}

impl Default for GlobalGamepadState {
    fn default() -> Self {
        Self {
            mutex_state: Arc::new(Mutex::new(GamepadState::new())),
            update_running: Arc::new(AtomicBool::new(false)),
        }
    }
}

#[tauri::command]
pub fn start_update(app_handle: AppHandle, state: tauri::State<'_, GlobalGamepadState>, user_id: u32) {
    // 如果已经在更新则不重复启动
    if state.update_running.swap(true, Ordering::SeqCst) {
        return;
    }
    let cancel_flag = state.update_running.clone();
    let mutex_state = Arc::clone(&state.mutex_state);
    let mut last_emit_time = chrono::Utc::now().timestamp_millis();
    // let mut prev_axes = std::collections::HashMap::new();
    tauri::async_runtime::spawn(async move {
        while cancel_flag.load(Ordering::SeqCst) {
            let mut gamepad_state = mutex_state.lock().unwrap();
            let gamepad = gamepad_state.get_xinput_gamepad(user_id);
            gamepad_state.record_polling_rate(user_id, true);
            if chrono::Utc::now().timestamp_micros() - last_emit_time >= 10000_i64 {
                app_handle
                    .emit("gamepads_info", gamepad.clone()).ok()
                    .expect("failed to emit gamepads_info");
                app_handle
                    .emit("polling_rate_log", gamepad_state.polling_rate_log.clone()).ok()
                    .expect("failed to emit polling_rate_log");
                app_handle
                    .emit("polling_rate_result", gamepad_state.polling_rate_result.clone()).ok()
                    .expect("failed to emit polling_rate_result");
                last_emit_time = chrono::Utc::now().timestamp_millis();
            }
        }
    });
}

#[tauri::command]
pub fn stop_update(state: tauri::State<'_, GlobalGamepadState>) {
    state.update_running.store(false, Ordering::SeqCst);
}

#[tauri::command]
pub fn get_gamepad_ids(state: tauri::State<'_, GlobalGamepadState>) -> Vec<u32> {
    let mut gamepad_state = state.mutex_state.lock().unwrap();
    let mut ids: Vec<u32> = gamepad_state.get_cur_gamepads().iter().map(|id| *id).collect();
    ids.sort();
    ids
}