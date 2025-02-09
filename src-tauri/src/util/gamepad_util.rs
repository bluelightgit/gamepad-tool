use std::{collections::{HashMap, HashSet}, mem, sync::Arc};
use std::sync::Mutex;
use std::thread::JoinHandle;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tauri::Emitter;
// use windows::Win32::UI::Input::XboxController::{
//     XInputGetBatteryInformation, XInputGetState, BATTERY_DEVTYPE, BATTERY_LEVEL_EMPTY, BATTERY_LEVEL_FULL, BATTERY_LEVEL_LOW, BATTERY_LEVEL_MEDIUM, XINPUT_BATTERY_INFORMATION, XINPUT_GAMEPAD_A, XINPUT_GAMEPAD_B, XINPUT_GAMEPAD_BACK, XINPUT_GAMEPAD_DPAD_DOWN, XINPUT_GAMEPAD_DPAD_LEFT, XINPUT_GAMEPAD_DPAD_RIGHT, XINPUT_GAMEPAD_DPAD_UP, XINPUT_GAMEPAD_LEFT_SHOULDER, XINPUT_GAMEPAD_LEFT_THUMB, XINPUT_GAMEPAD_RIGHT_SHOULDER, XINPUT_GAMEPAD_RIGHT_THUMB, XINPUT_GAMEPAD_START, XINPUT_GAMEPAD_X, XINPUT_GAMEPAD_Y, XINPUT_STATE, XUSER_MAX_COUNT
// };
use windows::Gaming::Input::{
    // GameControllerSwitchPosition, Gamepad as WgiGamepad, GamepadButtons, GamepadReading,
    RawGameController,
};
use uuid::Uuid;
use libm::atan2;
use crate::util::math_util::MathUtil;
use crate::util::input_wrapper::{RawInput, XInput};

const DEFAULT_FRAME_RATE: u32 = 60;
const MAX_LOG_SIZE: usize = 1000;
const CALCULATE_INTERVAL: usize = 100; // caluculate onece per 100 logs
const POLLING_RATE_RETRIEVE_INTERVAL: u64 = 1; // microsecond
const SDL_HARDWARE_BUS_USB: u32 = 0x03;
const MAX_R: f64 = 32767.0f64; // 最大圆半径
#[derive(Debug, Clone)]
pub struct GamepadState {
    xinput_state: XInput,
    frame_rate: u32,
    cur_gamepads: HashSet<u32>,
    polling_rate_log: HashMap<u32, Vec<PollingRateLog>>, // user_index, (timestamp, (x, y))
    polling_rate_result: HashMap<u32, PollingRateResult>, // user_index, (avg, min, max)
    math_utils: HashMap<u32, MathUtil>,
    direction_bins: HashMap<u32, (HashMap<Direction, u32>, HashMap<Direction, u32>)>, // user_index, (left(direction, max_radius), right(direction, max_radius))
}

impl GamepadState {
    pub fn new() -> Self {
        GamepadState {
            xinput_state: XInput::new(),
            frame_rate: 1000 / DEFAULT_FRAME_RATE,
            cur_gamepads: HashSet::new(),
            polling_rate_log: HashMap::new(),
            polling_rate_result: HashMap::new(),
            math_utils: HashMap::new(),
            direction_bins: HashMap::new(),
        }
    }

    pub fn set_frame_rate(&mut self, frame_rate: u32) {
        self.frame_rate = frame_rate;
    }

    /// 从 XInput 控制器状态构造 GamepadInfo
    pub fn from_xinput_state(
        &mut self,
        user_index: u32,   
    ) -> GamepadInfo {
        self.xinput_state.update(user_index);
        let gamepad = self.xinput_state.get_controller(user_index).unwrap();
        // 映射按钮
        let buttons = gamepad.buttons.iter().map(|(k, v)| {
            (
                k.to_string(),
                ButtonData {
                    button: k.to_string(),
                    is_pressed: v.is_pressed,
                    value: v.value as f64 / 255.0f64,
                },
            )
        }).collect::<HashMap<String, ButtonData>>();
        // 映射轴
        let axes = gamepad.axes.iter().map(|(k, v)| {
            (
                k.to_string(),
                AxisData {
                    axis: k.to_string(),
                    value: v.value as f64 / 32767.0f64,
                },
            )
        }).collect::<HashMap<String, AxisData>>();
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
            name,
            vendor_id: Some(vendor_id),
            product_id: Some(product_id),
            guid: uuid.to_string(),
            power_info: gamepad.power_info,
            axes,
            buttons,
        }
    }

    pub fn get_xinput_gamepads(&mut self) -> HashMap<u32, GamepadInfo> {
        let mut infos: HashMap<u32, GamepadInfo> = HashMap::new();
        for user_index in self.get_cur_gamepads().iter() {
            // 构造 GamepadInfo
            let gamepad_info = self.from_xinput_state(*user_index);
            self.cur_gamepads.insert(*user_index);
            // self.record_polling_rate(user_index, true);
            infos.insert(gamepad_info.id, gamepad_info);
        }

        infos
    }

    pub fn get_cur_gamepads(&mut self) -> HashSet<u32> {
        let cur: HashSet<u32> = self.xinput_state.all_device_id().iter().cloned().collect();
        if cur != self.cur_gamepads {
            self.cur_gamepads = cur.clone();
            self.polling_rate_log.clear();
            self.polling_rate_result.clear();
            self.math_utils.clear();
        }
        cur
    }

    pub fn get_polling_rate(&mut self, user_index: u32) -> PollingRateResult {
        let logs = self.polling_rate_log.get(&user_index).unwrap();
        let math_util = self.math_utils.entry(user_index).or_insert(MathUtil::new());
        let result = math_util.calc_frequency(logs.iter().map(|log| (log.timestamp as i64, log.xyxy.clone())).collect()).unwrap();
        let polling_rate_result = PollingRateResult {
            polling_rate_avg: result.0,
            polling_rate_min: result.1,
            polling_rate_max: result.2,
            avg_interval: result.3,
            avg_error_l: self.calc_avg_error(self.direction_bins.get(&user_index).unwrap().0.clone()),
            avg_error_r: self.calc_avg_error(self.direction_bins.get(&user_index).unwrap().1.clone()),
        };
        self.polling_rate_result.insert(user_index, polling_rate_result.clone());
        polling_rate_result
    }

    pub fn record_polling_rate(&mut self, user_index: u32, is_filter_duplicate: bool) -> PollingRateLog {
        let logs = self.polling_rate_log.entry(user_index).or_insert(Vec::new());
        let direction_log = self.direction_bins.entry(user_index).or_insert((HashMap::new(), HashMap::new()));
        self.xinput_state.update(user_index);
        let xyxy = self.xinput_state.get_axis_val().unwrap();
        let log = PollingRateLog {
            timestamp: chrono::Utc::now().timestamp_micros() as u64,
            xyxy,
        };

        if is_filter_duplicate && logs.len() > 0 && logs.last().unwrap().xyxy == log.xyxy {
            return log;
        } else {
            logs.push(log.clone());
            let theta_l = Direction::new(atan2(xyxy.1 as f64, xyxy.0 as f64), 0);
            let theta_r = Direction::new(atan2(xyxy.3 as f64, xyxy.2 as f64), 0);
            let r_l = ((xyxy.0 as f64).powi(2) + (xyxy.1 as f64).powi(2)).sqrt() as u32;
            let r_r = ((xyxy.2 as f64).powi(2) + (xyxy.3 as f64).powi(2)).sqrt() as u32;
            direction_log.0.entry(theta_l).or_insert(r_l);
            direction_log.1.entry(theta_r).or_insert(r_r);
        }

        // 限制日志长度
        if logs.len() > MAX_LOG_SIZE {
            logs.drain(0..=CALCULATE_INTERVAL);
            // logs.remove(0);
        }

        // 每100条记录计算一次
        if logs.len() % CALCULATE_INTERVAL == 0 {
            self.get_polling_rate(user_index);
        }

        log
    }

    fn calc_avg_error<T>(&self, dir_bin: HashMap<T, u32>) -> f64 {
        let length = dir_bin.len();
        let n = if length == 0 { 1 } else { length } as f64;
        let sum = dir_bin.iter().map(|(_, v)| v).sum::<u32>() as f64;
        1.0f64 - ((sum / n) / MAX_R)
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
    pub xyxy: (i16, i16, i16, i16),
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct PollingRateResult {
    pub polling_rate_avg: f64,
    pub polling_rate_min: f64,
    pub polling_rate_max: f64,
    pub avg_interval: f64,
    pub avg_error_l: f64,
    pub avg_error_r: f64,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Direction {
    int_part: i64,
    frac_part: u64,
    precision: u32, // 记录精度方便后续比较和输出
}

impl Direction {
    /// 使用给定精度（小数位数）拆分 f64 值的整数和小数部分
    fn new(val: f64, precision: u32) -> Direction {
        let int_part = val.trunc() as i64;
        let factor = 10u64.pow(precision);
        // 计算绝对值的小数部分，四舍五入再转为整数
        let frac_part = (val.fract().abs() * factor as f64).round() as u64;

        Direction {
            int_part,
            frac_part,
            precision,
        }
    }
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
        let mut last_emit_time = chrono::Utc::now().timestamp_micros();
        let mut prev_axes: HashMap<u32, (f64, f64, f64, f64)> = HashMap::new();
        loop {
            let mut gamepad_state = state.lock().unwrap();
            let gamepads = gamepad_state.get_xinput_gamepads();
            // app_handle
            //     .emit("gamepads_info", gamepads.clone()).ok()
            //     .expect("failed to emit gamepads_info");
            let gamepads_vec: Vec<u32> = gamepad_state.cur_gamepads.iter().cloned().collect();
            for i in gamepads_vec.iter() {
                gamepad_state.record_polling_rate(*i, true);
            }
            if chrono::Utc::now().timestamp_micros() - last_emit_time >= 10000_i64 {
                app_handle
                    .emit("gamepads_info", gamepads.clone()).ok()
                    .expect("failed to emit gamepads_info");
                app_handle
                    .emit("polling_rate_log", gamepad_state.polling_rate_log.clone()).ok()
                    .expect("failed to emit polling_rate_log");
                app_handle
                    .emit("polling_rate_result", gamepad_state.polling_rate_result.clone()).ok()
                    .expect("failed to emit polling_rate_result");
                last_emit_time = chrono::Utc::now().timestamp_millis();
            }
            let now_axes = gamepads.iter().map(|(k, v)|
                (*k, (v.axes.get("LeftThumbX").unwrap().value, v.axes.get("LeftThumbY").unwrap().value, v.axes.get("RightThumbX").unwrap().value, v.axes.get("RightThumbY").unwrap().value))).collect();
            if now_axes != prev_axes {
                prev_axes = now_axes;
                println!("{:?}", prev_axes);
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
