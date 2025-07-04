use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tauri::{AppHandle, Emitter};
// 添加专用线程池支持
use std::thread;

use crate::{
    util::gamepad_util::{polling_rate_log_to_output_log, PollingRateLog, PollingRateResult},
    GamepadState,
};
use tokio::time::{self, Duration};

const STANDBY_SLEEP_TIME: u64 = 10000;
const POLLING_RATE_MICROSECONDS: u64 = 125;

pub struct GlobalGamepadState {
    pub gamepad_state: Arc<GamepadState>,
    /// 当值为 true 时，表示更新任务正在运行
    pub update_running: Arc<AtomicBool>,
}

impl Default for GlobalGamepadState {
    fn default() -> Self {
        Self {
            gamepad_state: Arc::new(GamepadState::new()),
            update_running: Arc::new(AtomicBool::new(false)),
        }
    }
}

#[tauri::command]
pub fn stop_update(state: tauri::State<'_, GlobalGamepadState>) {
    state.update_running.store(false, Ordering::SeqCst);
}

#[tauri::command]
pub fn get_gamepad_ids(state: tauri::State<'_, GlobalGamepadState>) -> Vec<u32> {
    let gamepad_state = &state.gamepad_state;
    let mut ids: Vec<u32> = gamepad_state
        .get_cur_gamepads()
        .iter()
        .map(|id| *id)
        .collect();
    ids.sort();
    ids
}

#[tauri::command]
pub fn set_log_size(state: tauri::State<'_, GlobalGamepadState>, log_size: usize) {
    let gamepad_state = &state.gamepad_state;
    gamepad_state.set_log_size(log_size);
}

#[tauri::command]
pub fn clean_log(state: tauri::State<'_, GlobalGamepadState>) {
    let gamepad_state = &state.gamepad_state;
    gamepad_state.reset();
}

#[tauri::command]
pub fn start_update(
    app_handle: AppHandle,
    state: tauri::State<'_, GlobalGamepadState>,
    user_id: u32,
    frame_rate: u64,
    is_record_log: bool,
) {
    // 如果已经在更新则不重复启动
    if state.update_running.swap(true, Ordering::SeqCst) {
        return;
    }
    let cancel_flag = state.update_running.clone();
    let polling_cancel_flag = cancel_flag.clone();

    let gamepad_state = Arc::clone(&state.gamepad_state);

    // 高频轮询线程
    if is_record_log {
        let polling_gamepad_state = gamepad_state.clone();
        let polling_cancel_flag_clone = polling_cancel_flag.clone();
        
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let mut polling_interval = time::interval(Duration::from_micros(POLLING_RATE_MICROSECONDS));
                polling_interval.set_missed_tick_behavior(time::MissedTickBehavior::Burst);
                let mut standby_interval = time::interval(Duration::from_micros(STANDBY_SLEEP_TIME));

                loop {
                    if !polling_cancel_flag_clone.load(Ordering::SeqCst) {
                        break;
                    }
                    
                    // 快速检查是否有游戏手柄连接
                    let has_gamepads = !polling_gamepad_state.get_cur_gamepads().is_empty();

                    if !has_gamepads {
                        standby_interval.tick().await;
                        continue;
                    }

                    // 高频记录游戏手柄状态
                    polling_gamepad_state.record(user_id, true);

                    polling_interval.tick().await;
                }
            });
        });
    }

    // 数据发送任务
    tauri::async_runtime::spawn(async move {
        let mut emit_interval = time::interval(Duration::from_micros(1_000_000 / frame_rate));

        loop {
            if !cancel_flag.load(Ordering::SeqCst) {
                break;
            }

            // 获取数据并发送
            let data_result = {
                let gamepad = gamepad_state.get_xinput_gamepad(user_id);
                
                // 获取轮询率数据
                if let Some((polling_rate_log, polling_rate_result)) = gamepad_state.get_polling_data(user_id) {
                    Some((gamepad, polling_rate_log, polling_rate_result))
                } else {
                    Some((
                        gamepad,
                        vec![PollingRateLog::new()],
                        PollingRateResult::new(),
                    ))
                }
            };

            if let Some((gamepad, polling_rate_log, polling_rate_result)) = data_result {
                if gamepad.id == user_id {
                    let output_logs = polling_rate_log_to_output_log(&polling_rate_log);

                    let app_clone = app_handle.clone();
                    tokio::spawn(async move {
                        let _ = app_clone.emit("gamepads_info", gamepad);
                        let _ = app_clone.emit("polling_rate_log", output_logs);
                        let _ = app_clone.emit("polling_rate_result", polling_rate_result);
                    });
                }
            }

            emit_interval.tick().await;
        }
    });
}
