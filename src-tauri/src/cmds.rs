use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tauri::{AppHandle, Emitter};
// 添加专用线程池支持
use std::thread;

// 添加高精度计时器支持
#[cfg(target_os = "windows")]
use std::time::Instant;

use crate::{
    util::gamepad_util::{polling_rate_log_to_output_log, PollingRateLog, PollingRateResult},
    GamepadState,
};
use tokio::time::{self, Duration};

const STANDBY_SLEEP_TIME: u64 = 10000;
const POLLING_RATE_MICROSECONDS: u64 = 250;

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

    if is_record_log {
        let polling_gamepad_state = gamepad_state.clone();
        let polling_cancel_flag_clone = polling_cancel_flag.clone();

        thread::spawn(move || {
            let polling_duration = Duration::from_micros(POLLING_RATE_MICROSECONDS);
            let standby_duration = Duration::from_micros(STANDBY_SLEEP_TIME);

            loop {
                if !polling_cancel_flag_clone.load(Ordering::SeqCst) {
                    return;
                }

                // 高频记录游戏手柄状态
                if let Ok(_) = polling_gamepad_state.record(user_id, true) {
                    precise_sleep(polling_duration);
                } else {
                    precise_sleep(standby_duration);
                }
            }
        });
    }

    // 数据发送任务
    tauri::async_runtime::spawn(async move {
        let mut emit_interval = time::interval(Duration::from_micros(1_000_000 / frame_rate));

        loop {
            if !cancel_flag.load(Ordering::SeqCst) {
                return;
            }

            // 获取数据并发送
            if let Ok(gamepad) = gamepad_state.get_xinput_gamepad(user_id) {
                let app_clone = app_handle.clone();

                // 总是发送手柄基本信息
                let gamepad_clone = gamepad.clone();
                tokio::spawn(async move {
                    let _ = app_clone.emit("gamepads_info", gamepad_clone);
                });

                if let Some((polling_rate_log, polling_rate_result)) =
                    gamepad_state.get_polling_data(user_id)
                {
                    let app_clone = app_handle.clone();
                    // tokio::spawn(async move {
                    let _ = app_clone.emit(
                        "polling_rate_log",
                        polling_rate_log_to_output_log(&polling_rate_log),
                    );
                    let _ = app_clone.emit("polling_rate_result", polling_rate_result);
                    // });
                }
            }

            emit_interval.tick().await;
        }
    });
}

// 高精度睡眠函数，在Windows上使用更精准的计时
fn precise_sleep(duration: Duration) {
    #[cfg(target_os = "windows")]
    {
        let start = Instant::now();
        while start.elapsed() < duration {
            // 使用yield来避免忙等待
            thread::yield_now();
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        thread::sleep(duration);
    }
}
