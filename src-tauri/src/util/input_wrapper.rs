use std::collections::HashMap;
use std::fmt;
use windows::Win32::UI::Input::XboxController::{
    XInputGetBatteryInformation, XInputGetCapabilities, XInputGetState, BATTERY_DEVTYPE, BATTERY_LEVEL_EMPTY, BATTERY_LEVEL_FULL, BATTERY_LEVEL_LOW, BATTERY_LEVEL_MEDIUM, XINPUT_BATTERY_INFORMATION, XINPUT_CAPABILITIES, XINPUT_FLAG_ALL, XINPUT_GAMEPAD_A, XINPUT_GAMEPAD_B, XINPUT_GAMEPAD_BACK, XINPUT_GAMEPAD_BUTTON_FLAGS, XINPUT_GAMEPAD_DPAD_DOWN, XINPUT_GAMEPAD_DPAD_LEFT, XINPUT_GAMEPAD_DPAD_RIGHT, XINPUT_GAMEPAD_DPAD_UP, XINPUT_GAMEPAD_LEFT_SHOULDER, XINPUT_GAMEPAD_LEFT_THUMB, XINPUT_GAMEPAD_RIGHT_SHOULDER, XINPUT_GAMEPAD_RIGHT_THUMB, XINPUT_GAMEPAD_START, XINPUT_GAMEPAD_X, XINPUT_GAMEPAD_Y, XINPUT_STATE, XUSER_MAX_COUNT
};

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

pub trait RawInput<T> {
    fn new() -> Self;
    fn set_state(&mut self, state: T);
    fn get_state(&self) -> Option<T>;
    fn update(&mut self, id: u32) -> Option<T>;
    fn all_device_id(&self) -> Vec<u32>;
    fn get_controller(&self, id: u32) -> Option<Gamepad>;
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

#[derive(Clone, Debug)]
pub struct XInput {
    state: (XINPUT_STATE, XINPUT_BATTERY_INFORMATION)
}

impl RawInput<(XINPUT_STATE, XINPUT_BATTERY_INFORMATION)> for XInput {
    fn new() -> Self {
        XInput {
            state: (XINPUT_STATE::default(), XINPUT_BATTERY_INFORMATION::default())
        }
    }

    fn set_state(&mut self, state: (XINPUT_STATE, XINPUT_BATTERY_INFORMATION)) {
        self.state = state;
    }

    fn get_state(&self) -> Option<(XINPUT_STATE, XINPUT_BATTERY_INFORMATION)> {
        Some(self.state)
    }

    fn update(&mut self, id: u32) -> Option<(XINPUT_STATE, XINPUT_BATTERY_INFORMATION)> {
        let mut state = self.state;
        let result = unsafe { XInputGetState(id, &mut state.0) };
        unsafe { XInputGetBatteryInformation(id, BATTERY_DEVTYPE(0u8), &mut state.1); }
        if result == 0 {
            self.state = state;
        } else {
            self.state = (XINPUT_STATE::default(), XINPUT_BATTERY_INFORMATION::default());
        }
        Some(self.state)
    }

    fn all_device_id(&self) -> Vec<u32> {
        let mut device_ids = Vec::new();
        for i in 0..XUSER_MAX_COUNT {
            let mut state = self.state.0;
            let result = unsafe { XInputGetState(i, &mut state) };
            if result == 0 {
                device_ids.push(i);
            }
        }
        device_ids
    }

    fn get_controller(&self, id: u32) -> Option<Gamepad> {
        let state = self.state.0;
        let battery_state = self.state.1;
        let mut gamepad = Gamepad {
            id,
            name: "XInput Controller".to_string(),
            vendor_id: None,
            product_id: None,
            guid: "".to_string(),
            power_info: "".to_string(),
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
        gamepad.axes.insert(Axes::LeftThumbX, Axis { axis: Axes::LeftThumbX, value: state.Gamepad.sThumbLX });
        gamepad.axes.insert(Axes::LeftThumbY, Axis { axis: Axes::LeftThumbY, value: state.Gamepad.sThumbLY });
        gamepad.axes.insert(Axes::RightThumbX, Axis { axis: Axes::RightThumbX, value: state.Gamepad.sThumbRX });
        gamepad.axes.insert(Axes::RightThumbY, Axis { axis: Axes::RightThumbY, value: state.Gamepad.sThumbRY });
        BUTTONS_MAP.iter().for_each(|button_map| {
            let is_pressed = state.Gamepad.wButtons.contains(button_map.1);
            let value = if is_pressed { 255 } else { 0 };
            let btn = button_map.0.clone();
            gamepad.buttons.insert(btn.clone(), Button { button: btn, is_pressed, value });
        });
        gamepad.buttons.insert(Buttons::LeftTrigger, Button { button: Buttons::LeftTrigger, is_pressed: state.Gamepad.bLeftTrigger > 0, value: state.Gamepad.bLeftTrigger });
        gamepad.buttons.insert(Buttons::RightTrigger, Button { button: Buttons::RightTrigger, is_pressed: state.Gamepad.bRightTrigger > 0, value: state.Gamepad.bRightTrigger });
        Some(gamepad)
    }

    fn get_axis_val(&self) -> Option<(i16, i16, i16, i16)> {
        let state = self.state.0;
        Some((state.Gamepad.sThumbLX, state.Gamepad.sThumbLY, state.Gamepad.sThumbRX, state.Gamepad.sThumbRY))
    }
}
