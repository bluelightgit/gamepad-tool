use crate::util::input_wrapper::{RawInput, XInput};
use crate::util::math_util::MathUtil;
use libm::atan2;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;

const DEFAULT_LOG_SIZE: usize = 2000;
const CALCULATE_INTERVAL: usize = 100; // caluculate onece per 100 logs
const MAX_R: f64 = 32767.0f64; // 最大圆半径
const DEFAULT_DIR_PRECISION: u32 = 0;

#[derive(Debug)]
pub struct GamepadState {
    pub xinput_state: Arc<XInput>,
    pub cur_gamepads: Arc<Mutex<HashSet<u32>>>,
    pub memo: Arc<RwLock<HashMap<u32, Memo>>>,
}

#[derive(Debug, Clone)]
pub struct Memo {
    pub polling_rate_log: Vec<PollingRateLog>,
    pub polling_rate_result: PollingRateResult,
    pub direction_bins: (HashMap<Direction, f32>, HashMap<Direction, f32>),
    pub math_utils: MathUtil,
    pub log_size: usize,
    pub instant: Instant,
}

impl Memo {
    pub fn new() -> Self {
        Memo {
            polling_rate_log: Vec::with_capacity(DEFAULT_LOG_SIZE),
            polling_rate_result: PollingRateResult::new(),
            direction_bins: (HashMap::new(), HashMap::new()),
            math_utils: MathUtil::new(),
            log_size: DEFAULT_LOG_SIZE,
            instant: Instant::now(),
        }
    }

    pub fn reset(&mut self) {
        self.polling_rate_log.clear();
        self.polling_rate_result = PollingRateResult::new();
        self.direction_bins = (HashMap::new(), HashMap::new());
        self.math_utils = MathUtil::new();
        self.instant = Instant::now();
    }
}

unsafe impl Send for GamepadState {}

impl GamepadState {
    pub fn new() -> Self {
        GamepadState {
            xinput_state: Arc::new(XInput::new()),
            cur_gamepads: Arc::new(Mutex::new(HashSet::with_capacity(10))),
            memo: Arc::new(RwLock::new(HashMap::with_capacity(10))),
        }
    }

    /// 线程安全地从 XInput 控制器状态构造 GamepadInfo
    pub fn get_xinput_gamepad(&self, user_index: u32) -> Result<GamepadInfo, String> {
        let xinput_state = &self.xinput_state;
        let gamepad = if let Ok(_) = xinput_state.update(user_index) {
            xinput_state.get_controller(user_index).unwrap()
        } else {
            return Err(format!(
                "Failed to get controller for user index: {}",
                user_index
            ));
        };

        // 映射按钮
        let buttons = gamepad
            .buttons
            .iter()
            .map(|(k, v)| {
                (
                    k.to_string(),
                    ButtonData {
                        button: k.to_string(),
                        is_pressed: v.is_pressed,
                        value: v.value as f64 / 255.0f64,
                    },
                )
            })
            .collect::<HashMap<String, ButtonData>>();

        // 映射轴
        let axes = gamepad
            .axes
            .iter()
            .map(|(k, v)| {
                (
                    k.to_string(),
                    AxisData {
                        axis: k.to_string(),
                        value: v.value as f64 / 32767.0f64,
                    },
                )
            })
            .collect::<HashMap<String, AxisData>>();

        let vendor_id = 0;
        let product_id = 0;
        let name = format!("Xinput Controller {}", user_index);

        // 构造 GamepadInfo
        Ok(GamepadInfo {
            id: user_index,
            name,
            vendor_id: Some(vendor_id),
            product_id: Some(product_id),
            guid: "".to_string(),
            power_info: gamepad.power_info,
            axes,
            buttons,
        })
    }

    /// 线程安全地获取当前游戏手柄IDs
    pub fn get_cur_gamepads(&self) -> HashSet<u32> {
        let cur: HashSet<u32> = self.xinput_state.all_device_id().iter().cloned().collect();
        if !cur.is_empty() {
            if let Ok(mut cur_gamepads) = self.cur_gamepads.lock() {
                *cur_gamepads = cur.clone();
            }
        }
        cur
    }

    /// 线程安全地记录游戏手柄数据
    pub fn record(&self, user_index: u32, is_filter_duplicate: bool) -> Result<(), String> {
        // 获取轴值 - 先更新状态，然后获取轴值
        let xyxy = if let Ok(_) = self.xinput_state.update(user_index) {
            self.xinput_state.get_axis_val().unwrap_or((0, 0, 0, 0))
        } else {
            return Err("Failed to update XInput state".to_string());
        };

        // 记录数据
        if let Ok(mut memo_map) = self.memo.write() {
            let memo = memo_map.entry(user_index).or_insert(Memo::new());
            let logs = &mut memo.polling_rate_log;
            let direction_log = &mut memo.direction_bins;

            let log = PollingRateLog {
                timestamp: memo.instant.elapsed().as_micros() as u64,
                xyxy,
            };

            #[cfg(debug_assertions)]
            {
                println!("PollingRateLog: {:?}", log);
            }

            if is_filter_duplicate && !logs.is_empty() {
                if let Some(last_log) = logs.last() {
                    if last_log.xyxy == xyxy {
                        return Ok(());
                    }
                }
            }

            logs.push(log);
            let theta_l =
                Direction::new(atan2(xyxy.1 as f64, xyxy.0 as f64), DEFAULT_DIR_PRECISION);
            let theta_r =
                Direction::new(atan2(xyxy.3 as f64, xyxy.2 as f64), DEFAULT_DIR_PRECISION);
            let r_l = ((xyxy.0 as f64).powi(2) + (xyxy.1 as f64).powi(2)).sqrt() as f32;
            let r_r = ((xyxy.2 as f64).powi(2) + (xyxy.3 as f64).powi(2)).sqrt() as f32;
            direction_log
                .0
                .entry(theta_l)
                .and_modify(|r| *r = r_l.max(*r))
                .or_insert(r_l);
            direction_log
                .1
                .entry(theta_r)
                .and_modify(|r| *r = r_r.max(*r))
                .or_insert(r_r);

            // 限制日志长度
            if logs.len() > memo.log_size {
                logs.drain(0..=CALCULATE_INTERVAL);
            }

            // 每100条记录计算一次
            if logs.len() % CALCULATE_INTERVAL == 0 {
                get_performance_stat(memo);
            }

            return Ok(());
        }

        Err("Failed to lock memo map".to_string())
    }

    /// 线程安全地设置日志大小
    pub fn set_log_size(&self, log_size: usize) {
        if let Ok(mut memo_map) = self.memo.write() {
            memo_map.iter_mut().for_each(|(_, memo)| {
                memo.log_size = log_size;
                memo.polling_rate_log = Vec::with_capacity(log_size);
            });
        }
    }

    /// 线程安全地重置状态
    pub fn reset(&self) {
        if let Ok(mut memo_map) = self.memo.write() {
            memo_map.iter_mut().for_each(|(_, memo)| {
                memo.reset();
            });
        }
    }

    /// 无锁读取轮询率数据（使用读锁，不阻塞写入）
    pub fn get_polling_data(
        &self,
        user_id: u32,
    ) -> Option<(Vec<PollingRateLog>, PollingRateResult)> {
        if let Ok(memo_map) = self.memo.read() {
            memo_map.get(&user_id).map(|memo| {
                (
                    memo.polling_rate_log.clone(),
                    memo.polling_rate_result.clone(),
                )
            })
        } else {
            None
        }
    }
}

pub fn get_performance_stat(memo: &mut Memo) {
    let logs = &memo.polling_rate_log;
    let math_util = &mut memo.math_utils;
    let result = math_util
        .calc_frequency(
            &logs
                .iter()
                .map(|log| (log.timestamp as i64, &log.xyxy))
                .collect(),
        )
        .unwrap();
    memo.polling_rate_result = PollingRateResult {
        polling_rate_avg: result.0,
        polling_rate_min: result.1,
        polling_rate_max: result.2,
        avg_interval: result.3,
        avg_error_l: calc_avg_error(&memo.direction_bins.0),
        avg_error_r: calc_avg_error(&memo.direction_bins.1),
    };
}

fn calc_avg_error<T>(dir_bin: &HashMap<T, f32>) -> f64 {
    let length = dir_bin.len();
    let n = if length == 0 { 1 } else { length } as f64;
    let sum = dir_bin.iter().map(|(_, v)| v).sum::<f32>() as f64;
    (1.0f64 - ((sum / n) / MAX_R)).abs()
}

pub fn polling_rate_log_to_output_log(logs: &Vec<PollingRateLog>) -> Vec<OutputLog> {
    logs.iter()
        .map(|log| {
            let xyxy = log.xyxy;
            OutputLog {
                timestamp: log.timestamp,
                xyxy: (
                    xyxy.0 as f64 / MAX_R,
                    xyxy.1 as f64 / MAX_R,
                    xyxy.2 as f64 / MAX_R,
                    xyxy.3 as f64 / MAX_R,
                ),
            }
        })
        .collect()
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

impl PollingRateLog {
    pub fn new() -> Self {
        PollingRateLog {
            timestamp: 0,
            xyxy: (0, 0, 0, 0),
        }
    }
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct OutputLog {
    pub timestamp: u64,
    pub xyxy: (f64, f64, f64, f64),
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
