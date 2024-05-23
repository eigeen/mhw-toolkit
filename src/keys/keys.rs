use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use strum::{Display, EnumString};

use super::{
    keys_ll::{self, KeyBindEngine},
    GameKeyCode,
};

type KeyEventCallback = Box<dyn Fn(&KeyEvent) + 'static + Send + Sync>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumString, Display)]
pub enum KeyEventType {
    KeyDown,
    KeyUp,
    DoubleClick,
    Hold,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyEvent {
    pub keys: Vec<GameKeyCode>,
    pub event_type: KeyEventType,
}

struct DoubleClickBinding {
    pub interval: u64,
    pub callback: KeyEventCallback,
}

struct HoldBinding {
    pub interval: u64,
    pub callback: Arc<KeyEventCallback>,
}

impl From<keys_ll::KeyEvent> for KeyEvent {
    fn from(value: keys_ll::KeyEvent) -> Self {
        Self {
            keys: value.keys,
            event_type: match value.state {
                keys_ll::KeyStateLL::KeyDown => KeyEventType::KeyDown,
                keys_ll::KeyStateLL::KeyUp => KeyEventType::KeyUp,
            },
        }
    }
}

/// 高级按键绑定
pub struct KeyBind {
    keybind_engine: KeyBindEngine,
    hold_keys_binding: HashMap<Vec<GameKeyCode>, HoldBinding>,
    hold_keys_state: Arc<Mutex<HashMap<Vec<GameKeyCode>, (bool, Instant)>>>,
}

impl KeyBind {
    pub fn new() -> Self {
        Self {
            keybind_engine: KeyBindEngine::new(),
            hold_keys_binding: HashMap::new(),
            hold_keys_state: Arc::new(Mutex::new(HashMap::new())),
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

    /// 注册双击事件监听器
    pub fn add_double_click_listener<F>(&mut self, keys: &[GameKeyCode], interval: u64, f: F)
    where
        F: Fn(&KeyEvent) + 'static + Send + Sync,
    {
        let binding = DoubleClickBinding {
            interval,
            callback: Box::new(f),
        };
        let last_press_time = Mutex::new(Instant::now());
        self.add_key_listener(keys, move |event| {
            if event.event_type == KeyEventType::KeyDown {
                let now = Instant::now();
                let mut last = last_press_time.lock().unwrap();
                if now.duration_since(*last) < Duration::from_millis(binding.interval) {
                    let dc_event = KeyEvent {
                        keys: event.keys.clone(),
                        event_type: KeyEventType::DoubleClick,
                    };
                    (binding.callback)(&dc_event);
                } else {
                    *last = now;
                }
            }
        });
    }

    /// 注册长按事件监听器
    pub fn add_hold_listener<F>(&mut self, keys: &[GameKeyCode], interval: u64, f: F)
    where
        F: Fn(&KeyEvent) + 'static + Send + Sync,
    {
        let binding = HoldBinding {
            interval,
            callback: Arc::new(Box::new(f)),
        };
        self.hold_keys_binding.insert(keys.to_vec(), binding);
        let keys = keys.to_vec();
        let keys1 = keys.clone();

        let key_state = self.hold_keys_state.clone();
        self.add_key_listener(&keys, move |event| {
            let mut key_state = key_state.lock().unwrap();
            if event.event_type == KeyEventType::KeyDown {
                key_state.insert(keys1.clone(), (true, Instant::now()));
            }
            if event.event_type == KeyEventType::KeyUp {
                key_state.insert(keys1.clone(), (false, Instant::now()));
            }
        });
    }

    /// 注册一个单按键事件监听器
    fn add_single_key_listener<F>(&mut self, key: GameKeyCode, f: F)
    where
        F: Fn(&KeyEvent) + 'static + Send + Sync,
    {
        self.keybind_engine
            .add_key_listener(&[key], move |event_ll| {
                let event = event_ll.clone().into();
                f(&event);
            });
    }

    /// 注册一个多按键事件监听器
    fn add_multi_keys_listener<F>(&mut self, keys: &[GameKeyCode], f: F)
    where
        F: Fn(&KeyEvent) + 'static + Send + Sync,
    {
        self.keybind_engine.add_key_listener(keys, move |event_ll| {
            let event = event_ll.clone().into();
            f(&event);
        });
    }

    /// 更新按键数据
    pub fn update(&mut self) {
        self.keybind_engine.update();
        let mut hold_key_state = self.hold_keys_state.lock().unwrap();
        for (keycode, (is_pressing, press_time)) in hold_key_state.iter_mut() {
            if *is_pressing {
                if let Some(binding) = self.hold_keys_binding.get(keycode) {
                    let now = Instant::now();
                    if now.duration_since(*press_time) > Duration::from_millis(binding.interval) {
                        let hold_event = KeyEvent {
                            keys: keycode.clone(),
                            event_type: KeyEventType::Hold,
                        };
                        *is_pressing = false;
                        (binding.callback)(&hold_event)
                    }
                };
            }
        }
    }

    /// 更新按键数据
    pub async fn update_async(&mut self) {
        self.keybind_engine.update();
        let mut hold_key_state = self.hold_keys_state.lock().unwrap();
        for (keycode, (is_pressing, press_time)) in hold_key_state.iter_mut() {
            if *is_pressing {
                if let Some(binding) = self.hold_keys_binding.get(keycode) {
                    let now = Instant::now();
                    if now.duration_since(*press_time) > Duration::from_millis(binding.interval) {
                        let hold_event = KeyEvent {
                            keys: keycode.clone(),
                            event_type: KeyEventType::Hold,
                        };
                        *is_pressing = false;
                        (binding.callback)(&hold_event)
                    }
                };
            }
        }
    }
}
