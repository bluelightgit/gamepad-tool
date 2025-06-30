use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use tauri::{AppHandle, Emitter};

use crate::{
    util::gamepad_util::{polling_rate_log_to_output_log, PollingRateLog, PollingRateResult},
    GamepadState,
};
use tokio::sync::mpsc::UnboundedSender;
use tokio::time::{self, Duration};

const STANDBY_SLEEP_TIME: u64 = 10000;
const POLLING_RATE_MICROSECONDS: u64 = 250; // 0.25ms = 8000Hz 轮询率

pub struct GlobalGamepadState {
    pub mutex_state: Arc<Mutex<GamepadState>>,
    /// 当值为 true 时，表示更新任务正在运行
    pub update_running: Arc<AtomicBool>,
    /// WebSocket客户端列表，用于发送数据流
    pub clients: Arc<Mutex<Vec<UnboundedSender<String>>>>,
}

impl Default for GlobalGamepadState {
    fn default() -> Self {
        Self {
            mutex_state: Arc::new(Mutex::new(GamepadState::new())),
            update_running: Arc::new(AtomicBool::new(false)),
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

use crate::models::StreamData;

#[tauri::command]
pub fn start_update(
    app_handle: AppHandle,
    state: tauri::State<'_, GlobalGamepadState>,
    user_id: u32,
    frame_rate: u64,
) {
    // 如果已经在更新则不重复启动
    if state.update_running.swap(true, Ordering::SeqCst) {
        return;
    }
    let cancel_flag = state.update_running.clone();
    let mutex_state = Arc::clone(&state.mutex_state);
    let clients = state.clients.clone();

    // 创建专用的高频轮询任务 - 独立运行，不受UI影响
    let polling_cancel_flag = cancel_flag.clone();
    let polling_mutex_state = mutex_state.clone();
    tauri::async_runtime::spawn(async move {
        let mut polling_interval = time::interval(Duration::from_micros(POLLING_RATE_MICROSECONDS));
        // 设置为Burst模式以提高精度
        polling_interval.set_missed_tick_behavior(time::MissedTickBehavior::Burst);
        
        let mut standby_interval = time::interval(Duration::from_micros(STANDBY_SLEEP_TIME));

        loop {
            if !polling_cancel_flag.load(Ordering::SeqCst) {
                break;
            }

            // 快速检查是否有游戏手柄连接
            let has_gamepads = {
                // 使用try_lock避免阻塞，如果锁被占用就跳过这次轮询
                if let Ok(mut gamepad_state) = polling_mutex_state.try_lock() {
                    !gamepad_state.get_cur_gamepads().is_empty()
                } else {
                    // 如果无法获取锁，假设有游戏手柄继续轮询
                    true
                }
            };

            if !has_gamepads {
                standby_interval.tick().await;
                continue;
            }

            // 高频记录游戏手柄状态 - 使用try_lock避免阻塞
            if let Ok(mut gamepad_state) = polling_mutex_state.try_lock() {
                gamepad_state.record(user_id, true);
            }
            // 如果无法获取锁，跳过这次记录但继续轮询

            // 等待下一次轮询 - 这是关键，确保轮询间隔精确
            polling_interval.tick().await;
        }
    });

    // 创建独立的数据发送任务 - 低频率，避免影响轮询
    tauri::async_runtime::spawn(async move {
        let mut emit_interval = time::interval(Duration::from_micros(1_000_000 / frame_rate));
        
        loop {
            if !cancel_flag.load(Ordering::SeqCst) {
                break;
            }

            // 获取数据并发送
            let data_result = {
                // 使用try_lock减少阻塞时间
                if let Ok(mut gamepad_state) = mutex_state.try_lock() {
                    let gamepad = gamepad_state.get_xinput_gamepad(user_id);
                    let memo = gamepad_state.memo.get(&user_id);

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
                // 确保数据中包含当前选择的gamepad ID
                if gamepad.id == user_id {
                    let output_logs = polling_rate_log_to_output_log(&polling_rate_log);
                    
                    // 发送到WebSocket客户端（高优先级，非阻塞）
                    // let stream_data = StreamData {
                    //     gamepad: gamepad.clone(),
                    //     polling_rate_log: output_logs.clone(),
                    //     polling_rate_result: polling_rate_result.clone(),
                    // };

                    // if let Ok(json_data) = serde_json::to_string(&stream_data) {
                    //     // 异步发送WebSocket数据，不阻塞
                    //     let clients_clone = clients.clone();
                    //     let json_data_clone = json_data.clone();
                    //     tokio::spawn(async move {
                    //         if let Ok(mut clients_guard) = clients_clone.try_lock() {
                    //             clients_guard.retain(|tx| tx.send(json_data_clone.clone()).is_ok());
                    //         }
                    //     });
                    // }

                    // 异步发送Tauri事件，避免阻塞
                    let app_clone = app_handle.clone();
                    tokio::spawn(async move {
                        // 使用非阻塞方式发送事件
                        let _ = app_clone.emit("gamepads_info", gamepad);
                        let _ = app_clone.emit("polling_rate_log", output_logs);
                        let _ = app_clone.emit("polling_rate_result", polling_rate_result);
                    });
                }
            }

            // 等待下次发送间隔
            emit_interval.tick().await;
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
    let mut gamepad_state = state.mutex_state.lock().unwrap();
    gamepad_state.set_log_size(log_size);
}
