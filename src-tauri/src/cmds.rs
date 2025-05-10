use std::{sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex}, time::{Duration, Instant}};
use tauri::{AppHandle, Emitter};

use crate::{util::gamepad_util::{self, polling_rate_log_to_output_log, Memo, PollingRateLog, PollingRateResult}, GamepadState};

const DEFAULT_SLEEP_TIME: u64 = 1;
const STANDBY_SLEEP_TIME: u64 = 10000;

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
pub fn start_update(app_handle: AppHandle, state: tauri::State<'_, GlobalGamepadState>, user_id: u32, frame_rate: u64) {
    // 如果已经在更新则不重复启动
    if state.update_running.swap(true, Ordering::SeqCst) {
        return;
    }
    let cancel_flag = state.update_running.clone();
    let mutex_state = Arc::clone(&state.mutex_state);
    tauri::async_runtime::spawn(async move {
        // use intervals to schedule polling/emission and standby periods
        let mut emit_interval = tokio::time::interval(Duration::from_micros(1_000_000 / frame_rate));
        let mut standby_interval = tokio::time::interval(Duration::from_micros(STANDBY_SLEEP_TIME));
        loop {
            if !cancel_flag.load(Ordering::SeqCst) {
                break;
            }
            let (gamepad, polling_rate_log, polling_rate_result, cur_gamepads_len) = {
                let mut gamepad_state = mutex_state.lock().unwrap();
                let gamepad = gamepad_state.get_xinput_gamepad(user_id);
                gamepad_state.record(user_id, true);
                let memo = gamepad_state.memo.get(&user_id);
                
                let (polling_rate_log, 
                    polling_rate_result) = if let Some(memo) = memo {
                    (memo.polling_rate_log.clone(), memo.polling_rate_result.clone())
                } else {
                    (vec![PollingRateLog::new()], PollingRateResult::new())
                };
                (gamepad, polling_rate_log, polling_rate_result, gamepad_state.cur_gamepads.len())
            };
            if cur_gamepads_len == 0 {
                standby_interval.tick().await;
                continue;
            }
            emit_interval.tick().await;
            app_handle.emit("gamepads_info", gamepad.clone()).expect("failed to emit gamepads_info");
            app_handle.emit("polling_rate_log", polling_rate_log_to_output_log(&polling_rate_log)).expect("failed to emit polling_rate_log");
            app_handle.emit("polling_rate_result", polling_rate_result).expect("failed to emit polling_rate_result");
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

#[tauri::command]
pub fn set_log_size(state: tauri::State<'_, GlobalGamepadState>, log_size: usize) {
    let mut gamepad_state = state.mutex_state.lock().unwrap();
    gamepad_state.set_log_size(log_size);
}

