use std::{collections::HashMap, sync::Mutex};
use std::fmt;
use windows::Win32::UI::Input::XboxController::{
    XInputGetBatteryInformation, XInputGetState, BATTERY_DEVTYPE, BATTERY_LEVEL_EMPTY,
    BATTERY_LEVEL_FULL, BATTERY_LEVEL_LOW, BATTERY_LEVEL_MEDIUM, XINPUT_BATTERY_INFORMATION,
    XINPUT_GAMEPAD_A, XINPUT_GAMEPAD_B, XINPUT_GAMEPAD_BACK, XINPUT_GAMEPAD_BUTTON_FLAGS,
    XINPUT_GAMEPAD_DPAD_DOWN, XINPUT_GAMEPAD_DPAD_LEFT, XINPUT_GAMEPAD_DPAD_RIGHT,
    XINPUT_GAMEPAD_DPAD_UP, XINPUT_GAMEPAD_LEFT_SHOULDER, XINPUT_GAMEPAD_LEFT_THUMB,
    XINPUT_GAMEPAD_RIGHT_SHOULDER, XINPUT_GAMEPAD_RIGHT_THUMB, XINPUT_GAMEPAD_START,
    XINPUT_GAMEPAD_X, XINPUT_GAMEPAD_Y, XINPUT_STATE, XUSER_MAX_COUNT,
};

// rand is only used for debug virtual device
#[cfg(debug_assertions)]
use rand::Rng;

enum InputType {
    XInput,
    GameInput,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Buttons {
    A,
    B,
    X,
    Y,
    LeftShoulder,
    RightShoulder,
    Back,
    Start,
    LeftThumb,
    RightThumb,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    LeftTrigger,
    RightTrigger,
}

impl fmt::Display for Buttons {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Buttons::A => write!(f, "A"),
            Buttons::B => write!(f, "B"),
            Buttons::X => write!(f, "X"),
            Buttons::Y => write!(f, "Y"),
            Buttons::LeftShoulder => write!(f, "LeftShoulder"),
            Buttons::RightShoulder => write!(f, "RightShoulder"),
            Buttons::Back => write!(f, "Back"),
            Buttons::Start => write!(f, "Start"),
            Buttons::LeftThumb => write!(f, "LeftThumb"),
            Buttons::RightThumb => write!(f, "RightThumb"),
            Buttons::DPadUp => write!(f, "DPadUp"),
            Buttons::DPadDown => write!(f, "DPadDown"),
            Buttons::DPadLeft => write!(f, "DPadLeft"),
            Buttons::DPadRight => write!(f, "DPadRight"),
            Buttons::LeftTrigger => write!(f, "LeftTrigger"),
            Buttons::RightTrigger => write!(f, "RightTrigger"),
        }
    }
}

const BUTTONS_MAP: [(Buttons, XINPUT_GAMEPAD_BUTTON_FLAGS); 14] = [
    (Buttons::A, XINPUT_GAMEPAD_A),
    (Buttons::B, XINPUT_GAMEPAD_B),
    (Buttons::X, XINPUT_GAMEPAD_X),
    (Buttons::Y, XINPUT_GAMEPAD_Y),
    (Buttons::LeftShoulder, XINPUT_GAMEPAD_LEFT_SHOULDER),
    (Buttons::RightShoulder, XINPUT_GAMEPAD_RIGHT_SHOULDER),
    (Buttons::Back, XINPUT_GAMEPAD_BACK),
    (Buttons::Start, XINPUT_GAMEPAD_START),
    (Buttons::LeftThumb, XINPUT_GAMEPAD_LEFT_THUMB),
    (Buttons::RightThumb, XINPUT_GAMEPAD_RIGHT_THUMB),
    (Buttons::DPadUp, XINPUT_GAMEPAD_DPAD_UP),
    (Buttons::DPadDown, XINPUT_GAMEPAD_DPAD_DOWN),
    (Buttons::DPadLeft, XINPUT_GAMEPAD_DPAD_LEFT),
    (Buttons::DPadRight, XINPUT_GAMEPAD_DPAD_RIGHT),
];

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Axes {
    LeftThumbX,
    LeftThumbY,
    RightThumbX,
    RightThumbY,
}

impl fmt::Display for Axes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Axes::LeftThumbX => write!(f, "LeftThumbX"),
            Axes::LeftThumbY => write!(f, "LeftThumbY"),
            Axes::RightThumbX => write!(f, "RightThumbX"),
            Axes::RightThumbY => write!(f, "RightThumbY"),
        }
    }
}

pub trait RawInput<T, E> {
    fn new() -> Self;
    fn set_state(&self, state: T);
    fn get_state(&self) -> Result<T, E>;
    fn update(&self, id: u32) -> Result<T, E>;
    fn all_device_id(&self) -> Vec<u32>;
    fn get_controller(&self, id: u32) -> Result<Gamepad, E>;
    fn get_axis_val(&self) -> Option<(i16, i16, i16, i16)>;
}

#[derive(Debug, Clone)]
pub struct Axis {
    pub axis: Axes,
    pub value: i16,
}

#[derive(Debug, Clone)]
pub struct Button {
    pub button: Buttons,
    pub is_pressed: bool,
    pub value: u8,
}

#[derive(Debug, Clone)]
pub struct Gamepad {
    pub id: u32,
    pub name: String,
    pub vendor_id: Option<u16>,
    pub product_id: Option<u16>,
    pub guid: String,
    pub power_info: String,
    pub axes: HashMap<Axes, Axis>,
    pub buttons: HashMap<Buttons, Button>,
}

#[derive(Debug)]
pub struct XInput {
    state: Mutex<(XINPUT_STATE, XINPUT_BATTERY_INFORMATION)>,
}

impl RawInput<(XINPUT_STATE, XINPUT_BATTERY_INFORMATION), String> for XInput {
    fn new() -> Self {
        XInput {
            state: Mutex::new((
                XINPUT_STATE::default(),
                XINPUT_BATTERY_INFORMATION::default(),
            )),
        }
    }

    fn set_state(&self, state: (XINPUT_STATE, XINPUT_BATTERY_INFORMATION)) {
        if let Ok(mut current_state) = self.state.lock() {
            *current_state = state;
        }
    }

    fn get_state(&self) -> Result<(XINPUT_STATE, XINPUT_BATTERY_INFORMATION), String> {
        if let Ok(current_state) = self.state.lock() {
            return Ok(*current_state);
        }
        Err("Failed to lock state".to_string())
    }

    fn update(&self, id: u32) -> Result<(XINPUT_STATE, XINPUT_BATTERY_INFORMATION), String> {
        if let Ok(mut current_state) = self.state.lock() {
            let mut state = *current_state;
            let result = unsafe { XInputGetState(id, &mut state.0) };
            unsafe {
                XInputGetBatteryInformation(id, BATTERY_DEVTYPE(0u8), &mut state.1);
            }
            if result == 0 {
                // real device
                *current_state = state;
            } else {
                // no device => clear state
                *current_state = (
                    XINPUT_STATE::default(),
                    XINPUT_BATTERY_INFORMATION::default(),
                );
            }
            return Ok(state);
        }
        Err(format!("Failed to get state for device ID {}", id))
    }

    fn all_device_id(&self) -> Vec<u32> {
        let mut device_ids = Vec::new();
        for i in 0..XUSER_MAX_COUNT {
            if let Ok(state) = self.state.lock() {
                let mut test_state = state.0;
                let result = unsafe { XInputGetState(i, &mut test_state) };
                if result == 0 {
                    device_ids.push(i);
                }
            }
        }
        #[cfg(debug_assertions)]
        if device_ids.is_empty() {
            device_ids.push(0);
        }
        device_ids
    }

    fn get_controller(&self, id: u32) -> Result<Gamepad, String> {
        // 获取当前状态
        let (xi_state, batt) = if let Ok(state) = self.state.lock() {
            *state
        } else {
            return Err("Failed to lock state".to_string());
        };

        // real device mapping
        let battery_state = &batt;
        let mut gamepad = Gamepad {
            id,
            name: "XInput Controller".to_string(),
            vendor_id: None,
            product_id: None,
            guid: String::new(),
            power_info: String::new(),
            axes: HashMap::new(),
            buttons: HashMap::new(),
        };
        gamepad.power_info = match battery_state.BatteryLevel {
            BATTERY_LEVEL_EMPTY => "Empty".to_string(),
            BATTERY_LEVEL_LOW => "Low".to_string(),
            BATTERY_LEVEL_MEDIUM => "Medium".to_string(),
            BATTERY_LEVEL_FULL => "Full".to_string(),
            _ => "Unknown".to_string(),
        };
        gamepad.axes.insert(
            Axes::LeftThumbX,
            Axis {
                axis: Axes::LeftThumbX,
                value: xi_state.Gamepad.sThumbLX,
            },
        );
        gamepad.axes.insert(
            Axes::LeftThumbY,
            Axis {
                axis: Axes::LeftThumbY,
                value: xi_state.Gamepad.sThumbLY,
            },
        );
        gamepad.axes.insert(
            Axes::RightThumbX,
            Axis {
                axis: Axes::RightThumbX,
                value: xi_state.Gamepad.sThumbRX,
            },
        );
        gamepad.axes.insert(
            Axes::RightThumbY,
            Axis {
                axis: Axes::RightThumbY,
                value: xi_state.Gamepad.sThumbRY,
            },
        );
        BUTTONS_MAP.iter().for_each(|(btn, flag)| {
            let pressed = xi_state.Gamepad.wButtons.contains(*flag);
            let val = if pressed { 255 } else { 0 };
            gamepad.buttons.insert(
                btn.clone(),
                Button {
                    button: btn.clone(),
                    is_pressed: pressed,
                    value: val,
                },
            );
        });
        gamepad.buttons.insert(
            Buttons::LeftTrigger,
            Button {
                button: Buttons::LeftTrigger,
                is_pressed: xi_state.Gamepad.bLeftTrigger > 0,
                value: xi_state.Gamepad.bLeftTrigger,
            },
        );
        gamepad.buttons.insert(
            Buttons::RightTrigger,
            Button {
                button: Buttons::RightTrigger,
                is_pressed: xi_state.Gamepad.bRightTrigger > 0,
                value: xi_state.Gamepad.bRightTrigger,
            },
        );

        #[cfg(debug_assertions)]
        if gamepad.axes.values().all(|a| a.value == 0)
            && gamepad.buttons.values().all(|b| !b.is_pressed)
        {
            // debug virtual
            use std::f64::consts::PI;
            use std::time::{SystemTime, UNIX_EPOCH};

            use rand::rng;
            let t = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64();
            let f = t * 2.0 * PI / 5.0;
            let axis_val = |v: f64| (v * i16::MAX as f64) as i16;
            let mut gamepad = Gamepad {
                id,
                name: format!("Dummy {}", id),
                vendor_id: None,
                product_id: None,
                guid: String::new(),
                power_info: "Virtual".to_string(),
                axes: HashMap::new(),
                buttons: HashMap::new(),
            };
            // axes
            gamepad.axes.insert(
                Axes::LeftThumbX,
                Axis {
                    axis: Axes::LeftThumbX,
                    value: axis_val(f.cos()),
                },
            );
            gamepad.axes.insert(
                Axes::LeftThumbY,
                Axis {
                    axis: Axes::LeftThumbY,
                    value: axis_val(f.sin()),
                },
            );
            gamepad.axes.insert(
                Axes::RightThumbX,
                Axis {
                    axis: Axes::RightThumbX,
                    value: 0,
                },
            );
            gamepad.axes.insert(
                Axes::RightThumbY,
                Axis {
                    axis: Axes::RightThumbY,
                    value: 0,
                },
            );
            // buttons random
            let mut rng = rng();
            for (btn, _) in BUTTONS_MAP.iter() {
                let pressed = rng.random_bool(0.5);
                let val = if pressed { 255 } else { 0 };
                gamepad.buttons.insert(
                    btn.clone(),
                    Button {
                        button: btn.clone(),
                        is_pressed: pressed,
                        value: val,
                    },
                );
            }
            // triggers
            let trig = ((f.sin() * 0.5 + 0.5) * 255.0) as u8;
            gamepad.buttons.insert(
                Buttons::LeftTrigger,
                Button {
                    button: Buttons::LeftTrigger,
                    is_pressed: trig > 0,
                    value: trig,
                },
            );
            gamepad.buttons.insert(
                Buttons::RightTrigger,
                Button {
                    button: Buttons::RightTrigger,
                    is_pressed: trig > 0,
                    value: trig,
                },
            );
        }
        Ok(gamepad)
    }

    fn get_axis_val(&self) -> Option<(i16, i16, i16, i16)> {
        let state = if let Ok(state) = self.state.lock() {
            state.0
        } else {
            return None;
        };

        #[cfg(debug_assertions)]
        if state.Gamepad.sThumbLX == 0
            && state.Gamepad.sThumbLY == 0
            && state.Gamepad.sThumbRX == 0
            && state.Gamepad.sThumbRY == 0
        {
            use std::f64::consts::PI;
            use std::time::{SystemTime, UNIX_EPOCH};
            let t = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64();
            let f = t * 2.0 * PI / 5.0;
            let axis_val = |v: f64| (v * i16::MAX as f64) as i16;
            return Some((axis_val(f.cos()), axis_val(f.sin()), 0, 0));
        }
        Some((
            state.Gamepad.sThumbLX,
            state.Gamepad.sThumbLY,
            state.Gamepad.sThumbRX,
            state.Gamepad.sThumbRY,
        ))
    }
}
