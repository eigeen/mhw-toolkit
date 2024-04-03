use std::collections::{HashMap, HashSet};

use once_cell::sync::Lazy;
use strum_macros::FromRepr;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::KeyboardAndMouse::GetKeyState;
use windows::Win32::UI::WindowsAndMessaging::FindWindowW;
use windows::{core::PCWSTR, Win32::UI::WindowsAndMessaging::GetForegroundWindow};

use crate::game_export::XBOX_PAD_PTR;
use crate::util;

static MHW_LP_CLASS_NAME: Lazy<Vec<u16>> = Lazy::new(|| {
    "MT FRAMEWORK"
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect()
});
static MHW_LP_WINDOW_NAME: Lazy<Vec<u16>> = Lazy::new(|| {
    "MONSTER HUNTER: WORLD(421652)"
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect()
});

type KeyEventCallback = Box<dyn Fn(&KeyEvent) + 'static + Send + Sync>;
type HotkeyEventCallback = Box<dyn Fn(&HotkeyEvent) + 'static + Send + Sync>;
type Hotkey = Vec<GameKeyCode>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyState {
    KeyDown,
    KeyUp,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyEvent {
    pub key: GameKeyCode,
    pub state: KeyState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HotkeyEvent {
    pub keys: Hotkey,
    pub state: KeyState,
}

/// 按键绑定管理
///
/// 注册按键和绑定事件
///
/// 典型用例：
///
/// ```
/// fn main() {
///     let mut key_manager = KeybindManager::new();
///     key_manager.add_key_listener(&GameKeyCode::KeyboardMouse(VKeyCode::Tab), |event| {
///         println!("KeyEvent: {:?}", event);
///     });
///     key_manager.add_hotkey_listener(
///         &[
///             GameKeyCode::KeyboardMouse(VKeyCode::Ctrl),
///             GameKeyCode::KeyboardMouse(VKeyCode::A),
///         ],
///         |event| {
///             println!("HotkeyEvent: {:?}", event);
///         },
///     );
///     loop {
///         key_manager.update();
///         thread::sleep(Duration::from_millis(20));
///     }
/// }
/// ```
pub struct KeybindManager {
    controller_key_states: HashMap<ControllerCode, bool>,
    key_states: HashMap<GameKeyCode, bool>,
    registered_keys: HashSet<GameKeyCode>,
    key_callbacks: HashMap<GameKeyCode, Vec<KeyEventCallback>>,
    hotkey_callbacks: HashMap<Hotkey, Vec<HotkeyEventCallback>>,
}

impl KeybindManager {
    pub fn new() -> Self {
        Self {
            controller_key_states: HashMap::new(),
            key_states: HashMap::new(),
            registered_keys: HashSet::new(),
            key_callbacks: HashMap::new(),
            hotkey_callbacks: HashMap::new(),
        }
    }

    /// 注册一个按键事件监听器
    pub fn add_key_listener<F>(&mut self, key: &GameKeyCode, f: F)
    where
        F: Fn(&KeyEvent) + 'static + Send + Sync,
    {
        self.key_callbacks
            .entry(key.clone())
            .or_insert_with(Vec::new)
            .push(Box::new(f));
        self.registered_keys.insert(key.clone());
    }

    /// 注册一个热键按键事件监听器
    pub fn add_hotkey_listener<F>(&mut self, hotkey: &[GameKeyCode], f: F)
    where
        F: Fn(&HotkeyEvent) + 'static + Send + Sync,
    {
        self.hotkey_callbacks
            .entry(hotkey.to_vec())
            .or_insert_with(Vec::new)
            .push(Box::new(f));
        for key in hotkey.iter() {
            self.registered_keys.insert(key.clone());
        }
    }

    /// 更新按键数据，调用按键事件
    pub fn update(&mut self) {
        if !Self::check_window() {
            return;
        }
        self.update_controller();

        for key in self.registered_keys.iter() {
            let state = self.key_states.get(key).unwrap_or(&false);
            if self.check_key_down(key) {
                // 按键转换为按下状态
                if !state {
                    let key_event = KeyEvent {
                        key: *key,
                        state: KeyState::KeyDown,
                    };
                    // 单按键执行
                    self.execute_single_key(&key_event);
                    // 热键处理和执行
                    self.execute_hotkey(&key_event);
                }
                self.key_states.insert(*key, true);
            } else {
                // 按键转换为非按下状态
                if *state {
                    let key_event = KeyEvent {
                        key: *key,
                        state: KeyState::KeyUp,
                    };
                    // 单按键执行
                    self.execute_single_key(&key_event);
                    // 热键处理和执行
                    self.execute_hotkey(&key_event);
                }
                self.key_states.insert(*key, false);
            }
        }
    }

    // 执行单按键事件
    fn execute_single_key(&self, event: &KeyEvent) {
        if let Some(fns) = self.key_callbacks.get(&event.key) {
            fns.iter().for_each(|f| f(&event));
        }
    }

    // 执行热键事件
    fn execute_hotkey(&self, event: &KeyEvent) {
        for (hotkey, callbacks) in self.hotkey_callbacks.iter() {
            if hotkey.contains(&event.key) {
                // 需要除了当前按键之外的所有键全部处于按下状态
                let is_active = hotkey.iter().all(|key| {
                    if key == &event.key {
                        true
                    } else {
                        *self.key_states.get(key).unwrap_or(&false)
                    }
                });

                if is_active {
                    let hotkey_event = HotkeyEvent {
                        keys: hotkey.clone(),
                        state: event.state,
                    };
                    callbacks.iter().for_each(|f| f(&hotkey_event));
                }
            }
        }
    }

    /// 某个按键是否处于按下状态
    fn check_key_down(&self, gk: &GameKeyCode) -> bool {
        match gk {
            GameKeyCode::KeyboardMouse(vk) => self.check_keyboard_mouse(vk),
            GameKeyCode::Controller(ck) => self.check_controller(ck),
        }
    }

    /// 键鼠按键按下状态检查
    fn check_keyboard_mouse(&self, vk: &VKeyCode) -> bool {
        let state = unsafe { GetKeyState(vk.to_code()) };
        state < 0
    }

    /// 控制器按下状态检查
    fn check_controller(&self, ck: &ControllerCode) -> bool {
        match self.controller_key_states.get(&ck) {
            Some(state) => *state,
            None => false,
        }
    }

    /// 更新控制器按键状态
    fn update_controller(&mut self) {
        let mut up: bool;
        let mut down: bool;
        let mut left: bool;
        let mut right: bool;
        // LJoystick
        if Self::get_xbox_state(0xC44) > 0.0 {
            up = true;
            down = false;
        } else {
            up = false;
            down = true;
        }
        if Self::get_xbox_state(0xC40) > 0.0 {
            right = true;
            left = false;
        } else {
            right = false;
            left = true;
        }
        self.controller_key_states
            .insert(ControllerCode::LJoystickUp, up);
        self.controller_key_states
            .insert(ControllerCode::LJoystickDown, down);
        self.controller_key_states
            .insert(ControllerCode::LJoystickLeft, left);
        self.controller_key_states
            .insert(ControllerCode::LJoystickRight, right);
        // RJoystick
        if Self::get_xbox_state(0xC48) > 0.0 {
            up = true;
            down = false;
        } else {
            up = false;
            down = true;
        }
        if Self::get_xbox_state(0xC4C) > 0.0 {
            right = true;
            left = false;
        } else {
            right = false;
            left = true;
        }
        self.controller_key_states
            .insert(ControllerCode::RJoystickUp, up);
        self.controller_key_states
            .insert(ControllerCode::RJoystickDown, down);
        self.controller_key_states
            .insert(ControllerCode::RJoystickLeft, left);
        self.controller_key_states
            .insert(ControllerCode::RJoystickRight, right);
        // buttons
        self.controller_key_states.insert(
            ControllerCode::LJoystickPress,
            Self::get_xbox_state(0xC64) > 0.0,
        );
        self.controller_key_states.insert(
            ControllerCode::RJoystickPress,
            Self::get_xbox_state(0xC68) > 0.0,
        );
        self.controller_key_states
            .insert(ControllerCode::LT, Self::get_xbox_state(0xC88) > 0.0);
        self.controller_key_states
            .insert(ControllerCode::RT, Self::get_xbox_state(0xC8C) > 0.0);
        self.controller_key_states
            .insert(ControllerCode::LB, Self::get_xbox_state(0xC80) > 0.0);
        self.controller_key_states
            .insert(ControllerCode::RB, Self::get_xbox_state(0xC84) > 0.0);
        self.controller_key_states
            .insert(ControllerCode::Up, Self::get_xbox_state(0xC70) > 0.0);
        self.controller_key_states
            .insert(ControllerCode::Right, Self::get_xbox_state(0xC74) > 0.0);
        self.controller_key_states
            .insert(ControllerCode::Down, Self::get_xbox_state(0xC78) > 0.0);
        self.controller_key_states
            .insert(ControllerCode::Left, Self::get_xbox_state(0xC7C) > 0.0);
        self.controller_key_states
            .insert(ControllerCode::Y, Self::get_xbox_state(0xC90) > 0.0);
        self.controller_key_states
            .insert(ControllerCode::B, Self::get_xbox_state(0xC94) > 0.0);
        self.controller_key_states
            .insert(ControllerCode::A, Self::get_xbox_state(0xC98) > 0.0);
        self.controller_key_states
            .insert(ControllerCode::X, Self::get_xbox_state(0xC9C) > 0.0);
        self.controller_key_states
            .insert(ControllerCode::Window, Self::get_xbox_state(0xC60) > 0.0);
        self.controller_key_states
            .insert(ControllerCode::Menu, Self::get_xbox_state(0xC6C) > 0.0);
    }

    #[inline]
    fn get_xbox_state(offset: isize) -> f32 {
        util::get_value_with_offset(XBOX_PAD_PTR, &[offset]).unwrap_or(-1.0)
    }

    /// 获取游戏窗口句柄
    fn get_game_window() -> HWND {
        unsafe {
            FindWindowW(
                PCWSTR(MHW_LP_CLASS_NAME.as_ptr()),
                PCWSTR(MHW_LP_WINDOW_NAME.as_ptr()),
            )
        }
    }

    /// 检查当前活动窗口是否为游戏窗口
    fn check_window() -> bool {
        unsafe {
            let wnd = GetForegroundWindow();
            if wnd.0 == 0 {
                return false;
            }
            let mhd = Self::get_game_window();
            if mhd.0 == 0 {
                return false;
            }

            wnd == mhd
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameKeyCode {
    KeyboardMouse(VKeyCode),
    Controller(ControllerCode),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromRepr)]
pub enum ControllerCode {
    LJoystickUp,
    LJoystickRight,
    LJoystickDown,
    LJoystickLeft,
    LJoystickPress,
    RJoystickUp,
    RJoystickRight,
    RJoystickDown,
    RJoystickLeft,
    RJoystickPress,
    LT,
    RT,
    LB,
    RB,
    Up,
    Right,
    Down,
    Left,
    Y,
    B,
    A,
    X,
    Window,
    Menu,
}

/// 键码表
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromRepr)]
#[repr(i32)]
pub enum VKeyCode {
    LMouse = 1,
    RMouse = 2,
    Break = 3,
    MMouse = 4,
    BMouse = 5,
    FMouse = 6,
    Backspace = 8,
    Tab = 9,
    Enter = 13,
    Shift = 16,
    Ctrl = 17,
    Alt = 18,
    Pause = 19,
    CapsLock = 20,
    Esc = 27,
    Space = 32,
    PageUp = 33,
    PageDown = 34,
    End = 35,
    Home = 36,
    LeftArrow = 37,
    UpArrow = 38,
    RightArrow = 39,
    DownArrow = 40,
    PrintScreen = 44,
    Insert = 45,
    Delete = 46,
    Num0 = 48,
    Num1 = 49,
    Num2 = 50,
    Num3 = 51,
    Num4 = 52,
    Num5 = 53,
    Num6 = 54,
    Num7 = 55,
    Num8 = 56,
    Num9 = 57,
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    LWin = 91,
    RWin = 92,
    SelectKey = 93,
    Numpad0 = 96,
    Numpad1 = 97,
    Numpad2 = 98,
    Numpad3 = 99,
    Numpad4 = 100,
    Numpad5 = 101,
    Numpad6 = 102,
    Numpad7 = 103,
    Numpad8 = 104,
    Numpad9 = 105,
    Multiply = 106,
    Add = 107,
    Subtract = 109,
    DecimalPoint = 110,
    Divide = 111,
    F1 = 112,
    F2 = 113,
    F3 = 114,
    F4 = 115,
    F5 = 116,
    F6 = 117,
    F7 = 118,
    F8 = 119,
    F9 = 120,
    F10 = 121,
    F11 = 122,
    F12 = 123,
    NumLock = 144,
    ScrLk = 145,
    Semicolon = 186,
    EqualSign = 187,
    Comma = 188,
    Dash = 189,
    Period = 190,
    ForwardSlash = 191,
    GraveAccent = 192,
    OpenBracket = 219,
    BackSlash = 220,
    CloseBraket = 221,
    SingleQuote = 222,

    Other(i32),
}

impl ControllerCode {
    pub fn from(code: usize) -> Self {
        ControllerCode::from_repr(code).unwrap_or_else(|| ControllerCode::A)
    }
}

impl VKeyCode {
    pub fn from(code: i32) -> Self {
        match VKeyCode::from_repr(code) {
            Some(vkeycode) => vkeycode,
            None => VKeyCode::Other(code),
        }
    }

    pub fn to_code(&self) -> i32 {
        match self {
            VKeyCode::LMouse => 1,
            VKeyCode::RMouse => 2,
            VKeyCode::Break => 3,
            VKeyCode::MMouse => 4,
            VKeyCode::BMouse => 5,
            VKeyCode::FMouse => 6,
            VKeyCode::Backspace => 8,
            VKeyCode::Tab => 9,
            VKeyCode::Enter => 13,
            VKeyCode::Shift => 16,
            VKeyCode::Ctrl => 17,
            VKeyCode::Alt => 18,
            VKeyCode::Pause => 19,
            VKeyCode::CapsLock => 20,
            VKeyCode::Esc => 27,
            VKeyCode::Space => 32,
            VKeyCode::PageUp => 33,
            VKeyCode::PageDown => 34,
            VKeyCode::End => 35,
            VKeyCode::Home => 36,
            VKeyCode::LeftArrow => 37,
            VKeyCode::UpArrow => 38,
            VKeyCode::RightArrow => 39,
            VKeyCode::DownArrow => 40,
            VKeyCode::PrintScreen => 44,
            VKeyCode::Insert => 45,
            VKeyCode::Delete => 46,
            VKeyCode::Num0 => 48,
            VKeyCode::Num1 => 49,
            VKeyCode::Num2 => 50,
            VKeyCode::Num3 => 51,
            VKeyCode::Num4 => 52,
            VKeyCode::Num5 => 53,
            VKeyCode::Num6 => 54,
            VKeyCode::Num7 => 55,
            VKeyCode::Num8 => 56,
            VKeyCode::Num9 => 57,
            VKeyCode::A => 65,
            VKeyCode::B => 66,
            VKeyCode::C => 67,
            VKeyCode::D => 68,
            VKeyCode::E => 69,
            VKeyCode::F => 70,
            VKeyCode::G => 71,
            VKeyCode::H => 72,
            VKeyCode::I => 73,
            VKeyCode::J => 74,
            VKeyCode::K => 75,
            VKeyCode::L => 76,
            VKeyCode::M => 77,
            VKeyCode::N => 78,
            VKeyCode::O => 79,
            VKeyCode::P => 80,
            VKeyCode::Q => 81,
            VKeyCode::R => 82,
            VKeyCode::S => 83,
            VKeyCode::T => 84,
            VKeyCode::U => 85,
            VKeyCode::V => 86,
            VKeyCode::W => 87,
            VKeyCode::X => 88,
            VKeyCode::Y => 89,
            VKeyCode::Z => 90,
            VKeyCode::LWin => 91,
            VKeyCode::RWin => 92,
            VKeyCode::SelectKey => 93,
            VKeyCode::Numpad0 => 96,
            VKeyCode::Numpad1 => 97,
            VKeyCode::Numpad2 => 98,
            VKeyCode::Numpad3 => 99,
            VKeyCode::Numpad4 => 100,
            VKeyCode::Numpad5 => 101,
            VKeyCode::Numpad6 => 102,
            VKeyCode::Numpad7 => 103,
            VKeyCode::Numpad8 => 104,
            VKeyCode::Numpad9 => 105,
            VKeyCode::Multiply => 106,
            VKeyCode::Add => 107,
            VKeyCode::Subtract => 109,
            VKeyCode::DecimalPoint => 110,
            VKeyCode::Divide => 111,
            VKeyCode::F1 => 112,
            VKeyCode::F2 => 113,
            VKeyCode::F3 => 114,
            VKeyCode::F4 => 115,
            VKeyCode::F5 => 116,
            VKeyCode::F6 => 117,
            VKeyCode::F7 => 118,
            VKeyCode::F8 => 119,
            VKeyCode::F9 => 120,
            VKeyCode::F10 => 121,
            VKeyCode::F11 => 122,
            VKeyCode::F12 => 123,
            VKeyCode::NumLock => 144,
            VKeyCode::ScrLk => 145,
            VKeyCode::Semicolon => 186,
            VKeyCode::EqualSign => 187,
            VKeyCode::Comma => 188,
            VKeyCode::Dash => 189,
            VKeyCode::Period => 190,
            VKeyCode::ForwardSlash => 191,
            VKeyCode::GraveAccent => 192,
            VKeyCode::OpenBracket => 219,
            VKeyCode::BackSlash => 220,
            VKeyCode::CloseBraket => 221,
            VKeyCode::SingleQuote => 222,
            VKeyCode::Other(c) => *c,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vkeycode_from_named() {
        let code = 1;
        assert_eq!(VKeyCode::from(code), VKeyCode::LMouse);
    }

    #[test]
    fn test_vkeycode_from_other() {
        let code = 500;
        assert_eq!(VKeyCode::from(code), VKeyCode::Other(500));
    }

    #[test]
    fn test_vkeycode_to_code_named() {
        let vkeycode = VKeyCode::LMouse;
        assert_eq!(vkeycode.to_code(), 1);
    }

    #[test]
    fn test_vkeycode_to_code_other() {
        let vkeycode = VKeyCode::Other(500);
        assert_eq!(vkeycode.to_code(), 500);
    }
}
