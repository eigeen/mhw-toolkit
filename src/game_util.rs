use std::ffi::{c_char, CString};

use crate::{game_export, util};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum WeaponType {
    /// invalid
    #[default]
    Invalid = -1,
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
        match self {
            WeaponType::GreatSowrd => *other == 0,
            WeaponType::SwordAndShield => *other == 1,
            WeaponType::DualBlades => *other == 2,
            WeaponType::LongSword => *other == 3,
            WeaponType::Hammer => *other == 4,
            WeaponType::HuntingHorn => *other == 5,
            WeaponType::Lance => *other == 6,
            WeaponType::Gunlance => *other == 7,
            WeaponType::SwitchAxe => *other == 8,
            WeaponType::ChargeBlade => *other == 9,
            WeaponType::InsectGlaive => *other == 10,
            WeaponType::Bow => *other == 11,
            WeaponType::HeavyBowgun => *other == 12,
            WeaponType::LightBowgun => *other == 13,
            WeaponType::Invalid => false,
        }
    }
}

impl WeaponType {
    pub fn from_i32(id: i32) -> Self {
        match id {
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
            _ => WeaponType::Invalid,
        }
    }

    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

/// 特殊装备ID
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SpecializedToolType {
    #[default]
    None = -1,
    GhillieMantle = 0,
    TemporalMantle,
    HealthBooster,
    RocksteadyMantle,
    ChallengerMantle,
    VitalityMantle,
    FireproofMantle,
    WaterproofMantle,
    IceproofMantle,
    ThunderproofMantle,
    DragonproofMantle,
    CleanserBooster,
    GliderMantle,
    EvasionMantle,
    ImpactMantle,
    ApothecaryMantle,
    ImmunityMantle,
    AffinityBooster,
    BanditMantle,
    AssassinsHood,
}

impl PartialEq<i32> for SpecializedToolType {
    fn eq(&self, other: &i32) -> bool {
        *self as i32 == *other
    }
}

impl From<i32> for SpecializedToolType {
    fn from(value: i32) -> Self {
        match value {
            0 => SpecializedToolType::GhillieMantle,
            1 => SpecializedToolType::TemporalMantle,
            2 => SpecializedToolType::HealthBooster,
            3 => SpecializedToolType::RocksteadyMantle,
            4 => SpecializedToolType::ChallengerMantle,
            5 => SpecializedToolType::VitalityMantle,
            6 => SpecializedToolType::FireproofMantle,
            7 => SpecializedToolType::WaterproofMantle,
            8 => SpecializedToolType::IceproofMantle,
            9 => SpecializedToolType::DragonproofMantle,
            10 => SpecializedToolType::CleanserBooster,
            11 => SpecializedToolType::GliderMantle,
            12 => SpecializedToolType::EvasionMantle,
            13 => SpecializedToolType::ImpactMantle,
            14 => SpecializedToolType::ApothecaryMantle,
            15 => SpecializedToolType::ImmunityMantle,
            16 => SpecializedToolType::AffinityBooster,
            17 => SpecializedToolType::BanditMantle,
            18 => SpecializedToolType::AssassinsHood,
            _ => SpecializedToolType::None,
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
    chat_buffer: [u8; 128],
}

pub fn show_game_message(message: &str) {
    // 为了防止panic，通过检查玩家基址是否为空判断是否进入游戏场景
    // 可能存在不稳定性，待测试
    if util::get_ptr_with_offset(game_export::PLAYER_PTR, &[game_export::PLAYER_OFFSET])
        .map_or(true, |ptr| ptr.is_null())
    {
        return;
    };

    let show_message: extern "C" fn(*const usize, *const c_char, i32, i32, u8) =
        unsafe { std::mem::transmute(0x141A53400_i64) };
    let message_cstring = CString::new(message).unwrap();
    show_message(
        unsafe { *game_export::CHAT_MAIN_PTR as *const usize },
        message_cstring.as_ptr(),
        message.len() as i32,
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
    let mut buffer: [u8; 128] = [0; 128];
    let bytes_without_nul = message_cstring.as_bytes();
    if bytes_without_nul.len() >= 128 {
        buffer[0..127].copy_from_slice(&bytes_without_nul[0..127]);
    } else {
        buffer[0..bytes_without_nul.len()]
            .copy_from_slice(&bytes_without_nul[0..bytes_without_nul.len()]);
        buffer[bytes_without_nul.len()] = b'\0';
    }
    unsafe {
        (*chat).chat_buffer[0..128].copy_from_slice(&buffer);
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

    use crate::{game_export::{self, MESSAGE_BASE, MESSAGE_BODY_OFFSETS, MESSAGE_LEN_OFFSETS}, util::{self, get_ptr_with_offset}};

    use super::send_chat_message;

    /// 聊天消息发送工具
    ///
    /// 通过队列和锁防止高频发送吞消息的问题
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

    /// 接收聊天消息工具
    #[derive(Clone, Debug)]
    pub struct ChatMessageReceiver<const T: usize, const C: usize> {
        pub buffer: [u8; T],
        pub prefix: [u8; C],
        len: usize,
    }

    impl<const T: usize, const C: usize> ChatMessageReceiver<T, C> {
        /// 前缀过滤
        ///
        /// 将会通过设置的前缀过滤消息\
        /// 只接收满足前缀的消息\
        /// 否则将会消费所有消息（这可能会导致其他进程无法接收消息）
        pub const fn new(prefix: [u8; C]) -> Self {
            ChatMessageReceiver {
                buffer: [0; T],
                prefix,
                len: 0,
            }
        }

        pub fn get_message_len() -> usize {
            if let Some(msg_len) = util::get_ptr_with_offset(MESSAGE_BASE, MESSAGE_LEN_OFFSETS) {
                return unsafe { *msg_len } as usize;
            }
            0
        }
        pub fn read_command(&self) -> &str {
            std::str::from_utf8(&self.buffer[C - 1..self.len]).unwrap()
        }

        pub fn try_read_command(&mut self) -> bool {
            let msg_len = Self::get_message_len();
            if msg_len < C {
                return false;
            }
            if let Some(msg_body) = get_ptr_with_offset(MESSAGE_BASE, MESSAGE_BODY_OFFSETS) {
                let msg = unsafe { std::slice::from_raw_parts(msg_body as *const u8, msg_len) };
                let is_command = msg.starts_with(&self.prefix);
                if is_command {
                    unsafe {
                        self.buffer[C - 1..msg_len].copy_from_slice(&msg[C - 1..]);
                        self.len = msg_len;
                        std::ptr::write_bytes(msg_body as *mut u8, 0, C);
                    }
                    return true;
                }
            }
            false
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

pub use chat::ChatMessageReceiver;
pub use chat::ChatMessageSender;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weapon_type() {
        let longsword_id: i32 = 3;
        assert!(WeaponType::LongSword == longsword_id);
        assert!(WeaponType::LongSword.as_i32() == 3);
        assert!(WeaponType::from_i32(3) == WeaponType::LongSword);
    }
}
