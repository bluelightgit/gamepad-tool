use serde::Serialize;
use crate::util::gamepad_util::{GamepadInfo, OutputLog, PollingRateResult};

#[derive(Serialize)]
pub struct StreamData {
    pub gamepad: GamepadInfo,
    pub polling_rate_log: Vec<OutputLog>,
    pub polling_rate_result: PollingRateResult,
}
