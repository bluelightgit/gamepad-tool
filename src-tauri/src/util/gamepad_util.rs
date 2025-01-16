use std::{collections::{HashMap, HashSet}, sync::Arc};
use std::sync::Mutex;
use std::thread::JoinHandle;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tauri::Emitter;
use windows::Win32::UI::Input::XboxController::{
    XInputGetBatteryInformation, XInputGetCapabilities, XInputGetState, BATTERY_DEVTYPE, BATTERY_LEVEL_EMPTY, BATTERY_LEVEL_FULL, BATTERY_LEVEL_LOW, BATTERY_LEVEL_MEDIUM, XINPUT_BATTERY_INFORMATION, XINPUT_CAPABILITIES, XINPUT_FLAG_ALL, XINPUT_GAMEPAD_A, XINPUT_GAMEPAD_B, XINPUT_GAMEPAD_BACK, XINPUT_GAMEPAD_BUTTON_FLAGS, XINPUT_GAMEPAD_DPAD_DOWN, XINPUT_GAMEPAD_DPAD_LEFT, XINPUT_GAMEPAD_DPAD_RIGHT, XINPUT_GAMEPAD_DPAD_UP, XINPUT_GAMEPAD_LEFT_SHOULDER, XINPUT_GAMEPAD_LEFT_THUMB, XINPUT_GAMEPAD_RIGHT_SHOULDER, XINPUT_GAMEPAD_RIGHT_THUMB, XINPUT_GAMEPAD_START, XINPUT_GAMEPAD_X, XINPUT_GAMEPAD_Y, XINPUT_STATE, XUSER_MAX_COUNT
};
use windows::Gaming::Input::{
    // GameControllerSwitchPosition, Gamepad as WgiGamepad, GamepadButtons, GamepadReading,
    RawGameController,
};
use uuid::Uuid;

const DEFAULT_FRAME_RATE: u32 = 60;
const MAX_LOG_SIZE: usize = 3000;
const POLLING_RATE_RETRIEVE_INTERVAL: u64 = 1; // ms
const SDL_HARDWARE_BUS_USB: u32 = 0x03;
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GamepadState {
    frame_rate: u32,
    cur_gamepads: HashSet<u32>,
    polling_rate_log: HashMap<u32, Vec<PollingRateLog>>, // user_index, (timestamp, (x, y))
    polling_rate_result: HashMap<u32, PollingRateResult>, // user_index, (avg, min, max)
}

impl GamepadState {
    pub fn new() -> Self {
        GamepadState { 
            frame_rate: 1000 / DEFAULT_FRAME_RATE,
            cur_gamepads: HashSet::new(),
            polling_rate_log: HashMap::new(),
            polling_rate_result: HashMap::new(),
        }
    }

    pub fn set_frame_rate(&mut self, frame_rate: u32) {
        self.frame_rate = frame_rate;
    }

    /// 从 XInput 控制器状态构造 GamepadInfo
    pub fn from_xinput_state(
        &self,
        user_index: u32,   
    ) -> GamepadInfo {
        let mut state = XINPUT_STATE::default();
        unsafe { 
            XInputGetState(user_index, &mut state) 
        };
        let gamepad = state.Gamepad;
        // 映射按钮
        let buttons = [
            ("A", if gamepad.wButtons.contains(XINPUT_GAMEPAD_A) { 1.0 } else { 0.0 }),
            ("B", if gamepad.wButtons.contains(XINPUT_GAMEPAD_B) { 1.0 } else { 0.0 }),
            ("X", if gamepad.wButtons.contains(XINPUT_GAMEPAD_X) { 1.0 } else { 0.0 }),
            ("Y", if gamepad.wButtons.contains(XINPUT_GAMEPAD_Y) { 1.0 } else { 0.0 }),
            ("DPadUp", if gamepad.wButtons.contains(XINPUT_GAMEPAD_DPAD_UP) { 1.0 } else { 0.0 }),
            ("DPadDown", if gamepad.wButtons.contains(XINPUT_GAMEPAD_DPAD_DOWN) { 1.0 } else { 0.0 }),
            ("DPadLeft", if gamepad.wButtons.contains(XINPUT_GAMEPAD_DPAD_LEFT) { 1.0 } else { 0.0 }),
            ("DPadRight", if gamepad.wButtons.contains(XINPUT_GAMEPAD_DPAD_RIGHT) { 1.0 } else { 0.0 }),
            ("LeftShoulder", if gamepad.wButtons.contains(XINPUT_GAMEPAD_LEFT_SHOULDER) { 1.0 } else { 0.0 }),
            ("RightShoulder", if gamepad.wButtons.contains(XINPUT_GAMEPAD_RIGHT_SHOULDER) { 1.0 } else { 0.0 }),
            ("LeftThumb", if gamepad.wButtons.contains(XINPUT_GAMEPAD_LEFT_THUMB) { 1.0 } else { 0.0 }),
            ("RightThumb", if gamepad.wButtons.contains(XINPUT_GAMEPAD_RIGHT_THUMB) { 1.0 } else { 0.0 }),
            ("Start", if gamepad.wButtons.contains(XINPUT_GAMEPAD_START) { 1.0 } else { 0.0 }),
            ("Back", if gamepad.wButtons.contains(XINPUT_GAMEPAD_BACK) { 1.0 } else { 0.0 }),
            ("LeftTrigger", gamepad.bLeftTrigger as f64 / 255.0),
            ("RightTrigger", gamepad.bRightTrigger as f64 / 255.0),
        ]
        .iter()
        .map(|(name, value)| {
            (
                name.to_string(),
                ButtonData {
                    button: name.to_string(),
                    is_pressed: *value > 0.0,
                    value: value.clone(),
                },
            )
        })
        .collect::<HashMap<String, ButtonData>>();
        // 映射轴
        let axes = [
            ("LeftTrigger", gamepad.bLeftTrigger as f64 / 255.0),
            ("RightTrigger", gamepad.bRightTrigger as f64 / 255.0),
            ("LeftThumbX", gamepad.sThumbLX as f64 / 32767.0),
            ("LeftThumbY", gamepad.sThumbLY as f64 / 32767.0),
            ("RightThumbX", gamepad.sThumbRX as f64 / 32767.0),
            ("RightThumbY", gamepad.sThumbRY as f64 / 32767.0),
        ]
        .iter()
        .map(|(name, value)| {
            (
                name.to_string(),
                AxisData {
                    axis: name.to_string(),
                    value: *value,
                },
            )
        })
        .collect::<HashMap<String, AxisData>>();

        // battery
        let mut battery_info = XINPUT_BATTERY_INFORMATION::default();
        unsafe {
            XInputGetBatteryInformation(user_index, BATTERY_DEVTYPE(0u8), &mut battery_info);
        }
        let mut vendor_id = 0;
        let mut product_id = 0;
        let mut name = format!("Xinput Controller {}", user_index);
        let raw_game_controllers = RawGameController::RawGameControllers().unwrap();
        let uuid = match raw_game_controllers.Size().unwrap() == 0 || raw_game_controllers.GetAt(user_index).is_err() {
            true => Uuid::nil(),
            false => {
                let raw_game_controller = raw_game_controllers.GetAt(user_index).unwrap();
                vendor_id = raw_game_controller.HardwareVendorId().unwrap();
                product_id = raw_game_controller.HardwareProductId().unwrap();
                name = match raw_game_controller.DisplayName() {
                    Ok(hstring) => hstring.to_string_lossy(),
                    Err(_) => "unknown".to_string(),
                };
                let version = 0;
                let bustype = SDL_HARDWARE_BUS_USB.to_be();
                Uuid::from_fields(
                    bustype,
                    vendor_id,
                    0,
                    &[
                        (product_id >> 8) as u8,
                        product_id as u8,
                        0,
                        0,
                        (version >> 8) as u8,
                        version as u8,
                        0,
                        0,
                    ],
                )
            }
        };
        // 构造 GamepadInfo
        GamepadInfo {
            id: user_index,
            name: name,
            vendor_id: Some(vendor_id),
            product_id: Some(product_id),
            guid: uuid.to_string(),
            power_info: 
                match battery_info.BatteryLevel {
                    BATTERY_LEVEL_EMPTY => "Empty".to_string(),
                    BATTERY_LEVEL_LOW => "Low".to_string(),
                    BATTERY_LEVEL_MEDIUM => "Medium".to_string(),
                    BATTERY_LEVEL_FULL => "Full".to_string(),
                    _ => "Unknown".to_string(),
                },
            axes,
            buttons,
        }
    }

    pub fn get_xinput_gamepads(&mut self) -> HashMap<u32, GamepadInfo> {
        let mut infos: HashMap<u32, GamepadInfo> = HashMap::new();
        for user_index in 0..XUSER_MAX_COUNT {
            let mut state: XINPUT_STATE = Default::default();

            // 获取控制器状态
            let result = unsafe { XInputGetState(user_index, &mut state) };

            // 如果控制器未连接，跳过
            if result != 0 {
                continue;
            }

            let mut capabilities: XINPUT_CAPABILITIES = Default::default();
            let result = unsafe { XInputGetCapabilities(user_index, XINPUT_FLAG_ALL, &mut capabilities) };

            // 构造 GamepadInfo
            let gamepad_info = self.from_xinput_state(user_index);
            self.cur_gamepads.insert(user_index);
            // self.record_polling_rate(user_index, true);
            infos.insert(gamepad_info.id, gamepad_info);
        }

        infos
    }

    pub fn get_polling_rate(&mut self, user_index: u32) -> PollingRateResult {
        let logs = self.polling_rate_log.get(&user_index).unwrap();
        let mut sum = 0.0;
        let mut min = u64::MAX;
        let mut max = u64::MIN;
        let mut duplicate_count = 0;
        logs.windows(2).for_each(|pair| {
            let log = &pair[1];
            let prev_log = &pair[0];
            let delta = log.timestamp - prev_log.timestamp;
            sum += delta as f64;
            // 若前后数据点相同, 判定为重复并移除
            if &log.xxyy == &prev_log.xxyy {
                duplicate_count += 1;
                return;
            }
            if delta < min {
                min = delta;
            }
            if delta > max {
                max = delta;
            }
        });
        let avg = sum / ((logs.len() - duplicate_count) as f64);
        let polling_rate = PollingRateResult { // 每秒 / 间隔 = 轮询率
            polling_rate_avg: 1000000.0 / avg,
            polling_rate_min: 1000000.0 / max as f64,
            polling_rate_max: 1000000.0 / min as f64,
            avg_interval: avg / 1000.0, // ms
            drop_rate: (duplicate_count as f64) / (logs.len() as f64),
        };
        self.polling_rate_result.insert(user_index, polling_rate.clone());
        polling_rate
    }

    pub fn record_polling_rate(&mut self, user_index: u32, is_filter_duplicate: bool) -> PollingRateLog {
        let logs = self.polling_rate_log.entry(user_index).or_insert(Vec::new());
        let mut state = XINPUT_STATE::default();
        unsafe { XInputGetState(user_index, &mut state) };
        let gamepad = state.Gamepad;
        let xl = gamepad.sThumbLX;
        let yl = gamepad.sThumbLY;
        let xr = gamepad.sThumbRX;
        let yr = gamepad.sThumbRY;

        let log = PollingRateLog {
            timestamp: chrono::Utc::now().timestamp_micros() as u64,
            xxyy: (xl, yl, xr, yr),
        };

        if is_filter_duplicate && logs.len() > 0 && logs.last().unwrap().xxyy == log.xxyy {
            return log;
        } else {
            logs.push(log.clone());
        }

        // 限制日志长度
        if logs.len() > MAX_LOG_SIZE {
            logs.remove(0);
        }

        // 每100条记录计算一次
        if logs.len() % 100 == 0 {
            self.get_polling_rate(user_index);
        }

        log
    }
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct AxisData {
    pub axis: String,
    pub value: f64,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct ButtonData {
    pub button: String,
    pub is_pressed: bool,
    pub value: f64,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct GamepadInfo {
    pub id: u32,
    pub name: String,
    pub vendor_id: Option<u16>,
    pub product_id: Option<u16>,
    pub guid: String,
    pub power_info: String,
    pub axes: HashMap<String, AxisData>,
    pub buttons: HashMap<String, ButtonData>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct PollingRateLog {
    pub timestamp: u64,
    pub xxyy: (i16, i16, i16, i16),
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct PollingRateResult {
    pub polling_rate_avg: f64,
    pub polling_rate_min: f64,
    pub polling_rate_max: f64,
    pub avg_interval: f64,
    pub drop_rate: f64,
}

impl GamepadInfo {
    pub fn to_string(&self) -> String {
        format!(
            "Gamepad: {}\nVendor ID: {:?}\nProduct ID: {:?}\nPower Info: {}\nAxes: {:?}\nButtons: {:?}",
            self.name,
            self.vendor_id,
            self.product_id,
            self.power_info,
            self.axes,
            self.buttons
        )
    }
}

#[tauri::command]
pub fn start_update_thread(app_handle: tauri::AppHandle, state: Arc<Mutex<GamepadState>>) -> JoinHandle<()> {
    let handle = std::thread::spawn(move || {
        let mut last_emit_time = chrono::Utc::now().timestamp_millis();
        loop {
            let mut gamepad_state = state.lock().unwrap();
            let gamepads = gamepad_state.get_xinput_gamepads();
            app_handle
                .emit("gamepads_info", gamepads.clone()).ok()
                .expect("failed to emit gamepads_info");
            let gamepads_vec: Vec<u32> = gamepad_state.cur_gamepads.iter().cloned().collect();
            for i in gamepads_vec.iter() {
                gamepad_state.record_polling_rate(*i, true);
            }
            if last_emit_time - chrono::Utc::now().timestamp_micros() >= 100_i64 {
                app_handle
                    .emit("polling_rate_log", gamepad_state.polling_rate_log.clone()).ok()
                    .expect("failed to emit polling_rate_log");
                app_handle
                    .emit("polling_rate_result", gamepad_state.polling_rate_result.clone()).ok()
                    .expect("failed to emit polling_rate_result");
                last_emit_time = chrono::Utc::now().timestamp_millis();
            }

            std::thread::sleep(Duration::from_micros(POLLING_RATE_RETRIEVE_INTERVAL));
        }
    });
    handle
}

#[tauri::command]
pub fn get_polling_rate(user_index: u32, state: Arc<Mutex<GamepadState>>) -> PollingRateResult {
    let mut gamepad_state = state.lock().unwrap();
    gamepad_state.get_polling_rate(user_index)
}

#[tauri::command]
pub fn set_frame_rate(state: Arc<Mutex<GamepadState>>, frame_rate: u32) {
    let mut gamepad_state = state.lock().unwrap();
    gamepad_state.set_frame_rate(frame_rate);
}
