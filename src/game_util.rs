use std::ffi::{c_char, CString};

use crate::{game_export, util};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WeaponType {
    /// 大剑
    GreatSowrd = 0,
    /// 片手剑
    SwordAndShield = 1,
    /// 双刀
    DualBlades = 2,
    /// 太刀
    LongSword = 3,
    /// 大锤
    Hammer = 4,
    /// 狩猎笛
    HuntingHorn = 5,
    /// 长枪
    Lance = 6,
    /// 铳枪
    Gunlance = 7,
    /// 斩斧
    SwitchAxe = 8,
    /// 盾斧
    ChargeBlade = 9,
    /// 操虫棍
    InsectGlaive = 10,
    /// 弓
    Bow = 11,
    /// 重弩炮
    HeavyBowgun = 12,
    /// 轻弩炮
    LightBowgun = 13,
}

impl PartialEq<i32> for WeaponType {
    fn eq(&self, other: &i32) -> bool {
        *self as i32 == *other
    }
}

impl From<i32> for WeaponType {
    fn from(value: i32) -> Self {
        match value {
            0 => WeaponType::GreatSowrd,
            1 => WeaponType::SwordAndShield,
            2 => WeaponType::DualBlades,
            3 => WeaponType::LongSword,
            4 => WeaponType::Hammer,
            5 => WeaponType::HuntingHorn,
            6 => WeaponType::Lance,
            7 => WeaponType::Gunlance,
            8 => WeaponType::SwitchAxe,
            9 => WeaponType::ChargeBlade,
            10 => WeaponType::InsectGlaive,
            11 => WeaponType::Bow,
            12 => WeaponType::HeavyBowgun,
            13 => WeaponType::LightBowgun,
            _ => panic!("Invalid weapon type"),
        }
    }
}

#[repr(C)]
struct UGUIChat {
    vtable_ref: i64, // *mut i64
    unkptrs: [i64; 42],
    chat_index: i32,
    unk: i32,
    is_text_bar_visible: i32,
    space: u8,
    chat_buffer: [u8; 256],
}

pub fn show_game_message(message: &str) {
    let show_message: extern "C" fn(*const usize, *const c_char, i32, i32, u8) =
        unsafe { std::mem::transmute(0x141A53400_i64) };
    let message_cstring = CString::new(message).unwrap();
    show_message(
        unsafe { *game_export::CHAT_MAIN_PTR as *const usize },
        message_cstring.as_ptr(),
        -1,
        -1,
        0,
    )
}

pub fn send_chat_message(message: &str) {
    if message.len() == 0 {
        return;
    };

    let message_cstring = CString::new(message).unwrap();
    // 获取 UGUIChat 结构
    let chat = match util::get_ptr_with_offset(
        game_export::U_GUI_CHAT_BASE as *const UGUIChat,
        game_export::U_GUI_CHAT_STRUCT_OFFSETS,
    ) {
        Some(chat) => chat as *mut UGUIChat,
        None => return,
    };
    // 写入文本
    let mut buffer: [u8; 256] = [0; 256];
    let bytes_without_nul = message_cstring.as_bytes();
    if bytes_without_nul.len() > 256 {
        buffer[0..255].copy_from_slice(&bytes_without_nul[0..255]);
    } else {
        buffer[0..bytes_without_nul.len()]
            .copy_from_slice(&bytes_without_nul[0..bytes_without_nul.len()]);
        buffer[bytes_without_nul.len()] = b'\0';
    }
    unsafe {
        (*chat).chat_buffer[0..256].copy_from_slice(&buffer);
    }
    // 发送
    unsafe {
        match util::get_ptr_with_offset(
            game_export::U_GUI_CHAT_BASE as *const bool,
            game_export::U_GUI_CHAT_SEND_OFFSETS,
        ) {
            Some(_send_flag) => *(_send_flag as *mut bool) = true,
            None => return,
        };
    }
}

#[cfg(not(feature = "async-tokio"))]
mod chat {
    use std::{
        collections::VecDeque,
        sync::{Arc, Condvar, Mutex},
        thread,
        time::Duration,
    };

    use crate::{game_export, util};

    use super::send_chat_message;

    pub struct ChatMessageSender {
        queue: Arc<Mutex<VecDeque<String>>>,
        cond_var: Arc<Condvar>,
    }

    impl ChatMessageSender {
        pub fn new() -> Self {
            let instance = Self {
                queue: Arc::new(Mutex::new(VecDeque::new())),
                cond_var: Arc::new(Condvar::new()),
            };
            instance.start_background_sender();

            instance
        }

        /// 向队列追加一条消息
        pub fn send(&self, msg: &str) {
            let mut queue = self.queue.lock().unwrap();
            queue.push_back(msg.to_string());
            self.cond_var.notify_one();
        }

        pub fn start_background_sender(&self) {
            let queue_clone = self.queue.clone();
            let condvar_clone = self.cond_var.clone();

            thread::spawn(move || loop {
                let mut queue = queue_clone.lock().unwrap();
                while queue.is_empty() {
                    queue = condvar_clone.wait(queue).unwrap();
                }
                let msg = queue.pop_front().unwrap();
                drop(queue);

                while !Self::try_send(&msg) {
                    thread::sleep(Duration::from_millis(17));
                }
            });
        }

        /// 单次尝试发送消息
        pub fn try_send(msg: &str) -> bool {
            if Self::can_send() {
                send_chat_message(msg);
                true
            } else {
                false
            }
        }

        fn can_send() -> bool {
            // 如果是false则可以发送
            util::get_value_with_offset(
                game_export::U_GUI_CHAT_BASE as *const bool,
                game_export::U_GUI_CHAT_SEND_OFFSETS,
            )
            .map(|res| !res)
            .unwrap_or(false)
        }
    }
}

#[cfg(feature = "async-tokio")]
mod chat {

    use std::{collections::VecDeque, sync::Arc};

    use tokio::sync::{Mutex, Notify};
    use tokio::time::Duration;

    use crate::{game_export, util};

    use super::send_chat_message;

    pub struct ChatMessageSender {
        queue: Arc<Mutex<VecDeque<String>>>,
        notify: Arc<Notify>,
    }

    impl ChatMessageSender {
        pub fn new() -> Self {
            let instance = Self {
                queue: Arc::new(Mutex::new(VecDeque::new())),
                notify: Arc::new(Notify::new()),
            };
            instance.start_background_sender();

            instance
        }

        /// 向队列追加一条消息
        pub async fn send(&self, msg: &str) {
            let mut queue = self.queue.lock().await;
            queue.push_back(msg.to_string());
            self.notify.notify_one();
        }

        pub fn start_background_sender(&self) {
            let queue_clone = self.queue.clone();
            let notify_clone = self.notify.clone();

            tokio::spawn(async move {
                loop {
                    let mut queue = queue_clone.lock().await;
                    while queue.is_empty() {
                        notify_clone.notified().await;
                    }
                    let msg = queue.pop_front().unwrap();
                    drop(queue);

                    while !Self::try_send(&msg).await {
                        tokio::time::sleep(Duration::from_millis(17)).await;
                    }
                }
            });
        }

        /// 单次尝试发送消息
        pub async fn try_send(msg: &str) -> bool {
            if Self::can_send().await {
                send_chat_message(msg);
                true
            } else {
                false
            }
        }

        async fn can_send() -> bool {
            // 如果是false则可以发送
            util::get_value_with_offset(
                game_export::U_GUI_CHAT_BASE as *const bool,
                game_export::U_GUI_CHAT_SEND_OFFSETS,
            )
            .map(|res| !res)
            .unwrap_or(false)
        }
    }
}

pub use chat::ChatMessageSender;
