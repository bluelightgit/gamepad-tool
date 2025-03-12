use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use libm::atan2;
use crate::util::math_util::MathUtil;
use crate::util::input_wrapper::{RawInput, XInput};

const MAX_LOG_SIZE: usize = 1000;
const CALCULATE_INTERVAL: usize = 100; // caluculate onece per 100 logs
const MAX_R: f64 = 32767.0f64; // 最大圆半径
#[derive(Debug, Clone)]
pub struct GamepadState {
    pub xinput_state: XInput,
    pub cur_gamepads: HashSet<u32>,
    pub memo: HashMap<u32, Memo>,
}

#[derive(Debug, Clone)]
pub struct Memo {
    pub polling_rate_log: Vec<PollingRateLog>,
    pub polling_rate_result: PollingRateResult,
    pub direction_bins: (HashMap<Direction, u32>, HashMap<Direction, u32>),
    pub math_utils: MathUtil,
    pub log_size: usize,
}

impl Memo {
    pub fn new() -> Self {
        Memo {
            polling_rate_log: Vec::new(),
            polling_rate_result: PollingRateResult::new(),
            direction_bins: (HashMap::new(), HashMap::new()),
            math_utils: MathUtil::new(),
            log_size: MAX_LOG_SIZE,
        }
    }
}

impl GamepadState {
    pub fn new() -> Self {
        GamepadState {
            xinput_state: XInput::new(),
            cur_gamepads: HashSet::new(),
            memo: HashMap::new(),
        }
    }

    /// 从 XInput 控制器状态构造 GamepadInfo
    pub fn get_xinput_gamepad(
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
        let vendor_id = 0;
        let product_id = 0;
        let name = format!("Xinput Controller {}", user_index);
        
        // 构造 GamepadInfo
        GamepadInfo {
            id: user_index,
            name,
            vendor_id: Some(vendor_id),
            product_id: Some(product_id),
            guid: "".to_string(),
            power_info: gamepad.power_info,
            axes,
            buttons,
        }
    }

    pub fn get_xinput_gamepads(&mut self) -> HashMap<u32, GamepadInfo> {
        let mut infos: HashMap<u32, GamepadInfo> = HashMap::new();
        for user_index in self.get_cur_gamepads().iter() {
            // 构造 GamepadInfo
            let gamepad_info = self.get_xinput_gamepad(*user_index);
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
            self.memo.clear();
        }
        cur
    }

    pub fn get_polling_rate(&mut self, user_index: u32) -> PollingRateResult {
        let memo = self.memo.entry(user_index).or_insert(Memo::new());
        let logs = &memo.polling_rate_log;
        let math_util = &mut memo.math_utils;
        let result = math_util.calc_frequency(logs.iter().map(|log| (log.timestamp as i64, log.xyxy.clone())).collect()).unwrap();
        let polling_rate_result = PollingRateResult {
            polling_rate_avg: result.0,
            polling_rate_min: result.1,
            polling_rate_max: result.2,
            avg_interval: result.3,
            avg_error_l: calc_avg_error(memo.direction_bins.0.clone()),
            avg_error_r: calc_avg_error(memo.direction_bins.1.clone()),
        };
        memo.polling_rate_result = polling_rate_result.clone();
        polling_rate_result
    }

    pub fn record(&mut self, user_index: u32, is_filter_duplicate: bool) -> PollingRateLog {
        let memo = self.memo.entry(user_index).or_insert(Memo::new());
        let logs = &mut memo.polling_rate_log;
        let direction_log = &mut memo.direction_bins;
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
        if logs.len() > memo.log_size {
            logs.drain(0..=CALCULATE_INTERVAL);
            // logs.remove(0);
        }

        // 每100条记录计算一次
        if logs.len() % CALCULATE_INTERVAL == 0 {
            self.get_polling_rate(user_index);
        }

        log
    }

    pub fn get_record_log(&self, user_index: u32) -> Vec<PollingRateLog> {
        self.memo.get(&user_index).unwrap().polling_rate_log.clone()
    }

    pub fn set_log_size(&mut self, log_size: usize) {
        self.memo.iter_mut().for_each(|(_, memo)| {
            memo.log_size = log_size;
        });
    }
}

fn calc_avg_error<T>(dir_bin: HashMap<T, u32>) -> f64 {
    let length = dir_bin.len();
    let n = if length == 0 { 1 } else { length } as f64;
    let sum = dir_bin.iter().map(|(_, v)| v).sum::<u32>() as f64;
    1.0f64 - ((sum / n) / MAX_R)
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

impl PollingRateResult {
    pub fn new() -> Self {
        PollingRateResult {
            polling_rate_avg: 0.0,
            polling_rate_min: 0.0,
            polling_rate_max: 0.0,
            avg_interval: 0.0,
            avg_error_l: 0.0,
            avg_error_r: 0.0,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Direction {
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

