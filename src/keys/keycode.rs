use std::str::FromStr;

use strum::{EnumString, FromRepr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameKeyCode {
    KeyboardMouse(VKeyCode),
    Controller(ControllerCode),
}

#[allow(clippy::should_implement_trait, clippy::manual_strip)]
impl FromStr for GameKeyCode {
    type Err = String;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        if code.starts_with("Controller:") {
            // 解析为手柄按键
            let code = &code["Controller:".len()..];
            ControllerCode::from_str(code)
                .map(GameKeyCode::Controller)
                .map_err(|e| e.to_string())
        } else {
            // 解析为键盘按键
            VKeyCode::from_str(code)
                .map(GameKeyCode::KeyboardMouse)
                .map_err(|e| e.to_string())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString)]
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
    Back,
    Menu,
}

/// 键码表
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromRepr, EnumString)]
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
    fn test_parse_vkeycode() {
        assert_eq!(VKeyCode::from_str("A").unwrap(), VKeyCode::A);
        assert_eq!(VKeyCode::from_str("Numpad0").unwrap(), VKeyCode::Numpad0);
        assert_eq!(VKeyCode::from_str("C").unwrap(), VKeyCode::C);
    }
}
