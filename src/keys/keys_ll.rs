use std::collections::{HashMap, HashSet};

use windows::Win32::UI::Input::KeyboardAndMouse::GetKeyState;

use super::keycode::*;
use crate::game_export::XBOX_PAD_PTR;
use crate::util;

type KeyEventCallback = Box<dyn Fn(&KeyEvent) + 'static + Send + Sync>;
type Hotkey = Vec<GameKeyCode>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyStateLL {
    KeyDown,
    KeyUp,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyEvent {
    pub keys: Vec<GameKeyCode>,
    pub state: KeyStateLL,
}

/// 低级按键绑定管理
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
pub struct KeyBindEngine {
    controller_key_states: HashMap<ControllerCode, bool>,
    key_states: HashMap<GameKeyCode, bool>,
    registered_keys: HashSet<GameKeyCode>,
    key_callbacks: HashMap<GameKeyCode, Vec<KeyEventCallback>>,
    hotkey_callbacks: HashMap<Hotkey, Vec<KeyEventCallback>>,
}

impl KeyBindEngine {
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
    pub fn add_key_listener<F>(&mut self, keys: &[GameKeyCode], f: F)
    where
        F: Fn(&KeyEvent) + 'static + Send + Sync,
    {
        if keys.len() == 1 {
            self.add_single_key_listener(keys[0], f);
        } else if keys.len() > 1 {
            self.add_multi_keys_listener(keys, f)
        }
    }

    /// 注册一个单按键事件监听器
    fn add_single_key_listener<F>(&mut self, key: GameKeyCode, f: F)
    where
        F: Fn(&KeyEvent) + 'static + Send + Sync,
    {
        self.key_callbacks
            .entry(key.clone())
            .or_insert_with(Vec::new)
            .push(Box::new(f));
        self.registered_keys.insert(key.clone());
    }

    /// 注册一个多按键事件监听器
    fn add_multi_keys_listener<F>(&mut self, keys: &[GameKeyCode], f: F)
    where
        F: Fn(&KeyEvent) + 'static + Send + Sync,
    {
        self.hotkey_callbacks
            .entry(keys.to_vec())
            .or_insert_with(Vec::new)
            .push(Box::new(f));
        for key in keys.iter() {
            self.registered_keys.insert(key.clone());
        }
    }

    /// 更新按键数据，调用按键事件
    pub fn update(&mut self) {
        if !util::is_mhw_foreground() {
            return;
        }
        self.update_controller();

        for key in self.registered_keys.iter() {
            let state = self.key_states.get(key).unwrap_or(&false);
            let mut key_event = KeyEvent {
                keys: Vec::new(),
                state: KeyStateLL::KeyDown,
            };
            if self.check_key_down(key) {
                // 按键转换为按下状态
                if !state {
                    key_event.keys = vec![*key];
                    key_event.state = KeyStateLL::KeyDown;
                    // 单按键执行
                    self.execute_single_key(&key_event);
                    // 多按键处理和执行
                    self.execute_multi_keys(&key_event);
                }
                self.key_states.insert(*key, true);
            } else {
                // 按键转换为非按下状态
                if *state {
                    key_event.keys = vec![*key];
                    key_event.state = KeyStateLL::KeyUp;
                    // 单按键执行
                    self.execute_single_key(&key_event);
                    // 多按键处理和执行
                    self.execute_multi_keys(&key_event);
                }
                self.key_states.insert(*key, false);
            }
        }
    }

    // 执行单按键事件
    fn execute_single_key(&self, event: &KeyEvent) {
        if let Some(fns) = self.key_callbacks.get(&event.keys[0]) {
            fns.iter().for_each(|f| f(&event));
        }
    }

    // 执行热键事件
    fn execute_multi_keys(&self, event: &KeyEvent) {
        for (hotkey, callbacks) in self.hotkey_callbacks.iter() {
            if hotkey.contains(&event.keys[0]) {
                // 需要除了当前按键之外的所有键全部处于按下状态
                let is_active = hotkey.iter().all(|key| {
                    if key == &event.keys[0] {
                        true
                    } else {
                        *self.key_states.get(key).unwrap_or(&false)
                    }
                });

                if is_active {
                    let hotkey_event = KeyEvent {
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
            .insert(ControllerCode::Back, Self::get_xbox_state(0xC60) > 0.0);
        self.controller_key_states
            .insert(ControllerCode::Menu, Self::get_xbox_state(0xC6C) > 0.0);
    }

    #[inline]
    fn get_xbox_state(offset: isize) -> f32 {
        util::get_value_with_offset(XBOX_PAD_PTR, &[offset]).unwrap_or(-1.0)
    }
}
