use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
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
const POLLING_RATE_MICROSECONDS: u64 = 250; // 0.25ms = 4000Hz 轮询率

pub struct GlobalGamepadState {
    pub pad_state: GamepadState,
    /// 当值为 true 时，表示更新任务正在运行
    pub update_running: Arc<AtomicBool>,
}

impl Default for GlobalGamepadState {
    fn default() -> Self {
        Self {
            pad_state: GamepadState::new(),
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
    let mut ids: Vec<u32> = state.pad_state.get_cur_gamepads().iter()
        .map(|id| *id)
        .collect();
    ids.sort();
    ids
}

#[tauri::command]
pub fn set_log_size(state: tauri::State<'_, GlobalGamepadState>, log_size: usize) {
    let mut gamepad_state = state.pad_state.set_log_size(log_size);
}

#[tauri::command]
pub fn clean_log(state: tauri::State<'_, GlobalGamepadState>) {
    state.pad_state.reset();
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

    let pad_state = state.pad_state;

    let polling_pad_state = pad_state.clone();
    if is_record_log {
        let polling_cancel_flag_clone = polling_cancel_flag.clone();
        thread::spawn(move || {
            use std::time::{Duration as StdDuration, Instant};

            let polling_interval = StdDuration::from_micros(POLLING_RATE_MICROSECONDS);
            let standby_interval = StdDuration::from_micros(STANDBY_SLEEP_TIME);

            let mut next_poll_time = Instant::now() + polling_interval;
            let mut next_standby_time = Instant::now() + standby_interval;

            loop {
                if !polling_cancel_flag_clone.load(Ordering::SeqCst) {
                    break;
                }

                // 快速检查是否有游戏手柄连接
                let has_gamepads = {
                    if let Ok(mut gamepad_state) = polling_pad_state.try_lock() {
                        !gamepad_state.get_cur_gamepads().is_empty()
                    } else {
                        true // 如果无法获取锁，假设有游戏手柄继续轮询
                    }
                };

                if !has_gamepads {
                    // 没有游戏手柄时使用较低频率的待机模式
                    let now = Instant::now();
                    if now < next_standby_time {
                        thread::sleep(next_standby_time - now);
                    }
                    next_standby_time = Instant::now() + standby_interval;
                    continue;
                }

                // 高频记录游戏手柄状态
                if let Ok(mut gamepad_state) = polling_pad_state.try_lock() {
                    gamepad_state.record(user_id, true);
                }

                // 精确控制轮询间隔
                let now = Instant::now();
                if now < next_poll_time {
                    thread::sleep(next_poll_time - now);
                }
                next_poll_time = Instant::now() + polling_interval;
            }
        });
    }

    // 数据发送任务仍使用异步方式
    tauri::async_runtime::spawn(async move {
        let mut emit_interval = time::interval(Duration::from_micros(1_000_000 / frame_rate));

        loop {
            if !cancel_flag.load(Ordering::SeqCst) {
                break;
            }

            // 获取数据并发送
            let data_result = {
                if let Ok(mut gamepad_state) = pad_state.try_lock() {
                    let gamepad = gamepad_state.get_xinput_gamepad(user_id);
                    let memo_map = gamepad_state.memo.try_lock().unwrap();
                    let memo = memo_map.get(&user_id);

                    let (polling_rate_log, polling_rate_result) = if let Some(memo) = memo {
                        (
                            memo.polling_rate_log.clone(),
                            memo.polling_rate_result.clone(),
                        )
                    } else {
                        (vec![PollingRateLog::new()], PollingRateResult::new())
                    };
                    Some((gamepad, polling_rate_log, polling_rate_result))
                } else {
                    None
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
