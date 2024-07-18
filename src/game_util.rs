use std::ffi::{c_char, CString};

use crate::game_export;
use crate::utils;

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

/// # Deprecated
///
/// 请优先使用 `show_system_message`
#[deprecated]
pub fn show_game_message(message: &str) {
    // 为了防止panic，通过检查玩家基址是否为空判断是否进入游戏场景
    // 可能存在不稳定性，待测试
    if utils::get_ptr_with_offset(game_export::PLAYER_PTR, &[game_export::PLAYER_OFFSET])
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

#[repr(u8)]
pub enum SystemMessageColor {
    Blue = 0,
    Purple = 1,
}

/// 在游戏右侧对话框显示系统消息
///
/// 颜色：蓝框或紫框
pub fn show_system_message(message: &str, color: SystemMessageColor) {
    // 为了防止panic，通过检查玩家基址是否为空判断是否进入游戏场景
    // 可能存在不稳定性，待测试
    if utils::get_ptr_with_offset(game_export::PLAYER_PTR, &[game_export::PLAYER_OFFSET])
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
        color as u8,
    )
}

pub fn send_chat_message(message: &str) {
    if message.is_empty() {
        return;
    };

    let message_cstring = CString::new(message).unwrap();
    // 获取 UGUIChat 结构
    let chat = match utils::get_ptr_with_offset(
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
        if let Some(send_flag) = utils::get_ptr_with_offset(
            game_export::U_GUI_CHAT_BASE as *const bool,
            game_export::U_GUI_CHAT_SEND_OFFSETS,
        ) {
            *(send_flag.cast_mut()) = true;
        }
    }
}

pub mod chat {
    use std::{
        collections::VecDeque,
        sync::{Arc, Condvar, Mutex},
        thread,
        time::Duration,
    };

    use crate::{game_export, utils};

    use super::send_chat_message;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum SendTarget {
        Default,
        Quest,
        OutsideQuest,
        /// 指定玩家SteamID
        Specified(u64),
        AtHome,
        All,
    }

    impl From<i32> for SendTarget {
        fn from(value: i32) -> Self {
            match value {
                0 => SendTarget::Quest,
                1 => SendTarget::OutsideQuest,
                2 => SendTarget::Specified(0),
                3 => SendTarget::AtHome,
                4 => SendTarget::All,
                _ => SendTarget::Default,
            }
        }
    }

    impl SendTarget {
        pub fn as_i32(&self) -> i32 {
            match self {
                SendTarget::Default => -1,
                SendTarget::Quest => 0,
                SendTarget::OutsideQuest => 1,
                SendTarget::Specified(_) => 2,
                SendTarget::AtHome => 3,
                SendTarget::All => 4,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct ChatMessage {
        msg: String,
        target: SendTarget,
    }

    /// 聊天消息发送工具
    ///
    /// 通过队列和锁防止高频发送吞消息的问题
    pub struct ChatMessageSender {
        queue: Arc<Mutex<VecDeque<ChatMessage>>>,
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
            queue.push_back(ChatMessage {
                msg: msg.to_string(),
                target: SendTarget::Default,
            });
            self.cond_var.notify_one();
        }

        /// 向队列追加一条消息，附加高级选项
        pub fn send_ex(&self, msg: &str, target: SendTarget) {
            let mut queue = self.queue.lock().unwrap();
            queue.push_back(ChatMessage {
                msg: msg.to_string(),
                target,
            });
            self.cond_var.notify_one();
        }

        fn start_background_sender(&self) {
            let queue_clone = self.queue.clone();
            let condvar_clone = self.cond_var.clone();

            thread::spawn(move || loop {
                let mut queue = queue_clone.lock().unwrap();
                while queue.is_empty() {
                    queue = condvar_clone.wait(queue).unwrap();
                }
                let msg = queue.pop_front().unwrap();
                drop(queue);

                // 设置发送目标
                if msg.target != SendTarget::Default {
                    if let SendTarget::Specified(player_id) = msg.target {
                        Self::set_send_target_player(player_id);
                    }
                    Self::set_send_target(msg.target);
                }
                while !Self::try_send(&msg.msg) {
                    // 自旋等待重发
                    thread::sleep(Duration::from_millis(50));
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

        ///  获取当前设置的发送目标
        pub fn get_current_send_target() -> SendTarget {
            let send_target_i32 = utils::get_value_with_offset(
                game_export::U_GUI_CHAT_BASE as *const i32,
                game_export::U_GUI_CHAT_SEND_TARGET_OFFSETS,
            )
            .unwrap_or(SendTarget::All.as_i32());
            match send_target_i32 {
                2 => SendTarget::Specified(Self::get_current_send_player_target().unwrap_or(0)),
                other => SendTarget::from(other),
            }
        }

        /// 获取当前设置发送目标的玩家SteamID
        ///
        /// 仅在SendTarget::Specified下有效
        fn get_current_send_player_target() -> Option<u64> {
            utils::get_value_with_offset(
                game_export::U_GUI_CHAT_BASE as *const u64,
                game_export::U_GUI_CHAT_SEND_TARGET_PLAYER_OFFSETS,
            )
        }

        fn can_send() -> bool {
            // 如果是false则可以发送
            utils::get_value_with_offset(
                game_export::U_GUI_CHAT_BASE as *const bool,
                game_export::U_GUI_CHAT_SEND_OFFSETS,
            )
            .map(|res| !res)
            .unwrap_or(false)
        }

        fn set_send_target(target: SendTarget) {
            if let Some(send_target_i32) = utils::get_ptr_with_offset(
                game_export::U_GUI_CHAT_BASE as *const i32,
                game_export::U_GUI_CHAT_SEND_TARGET_OFFSETS,
            ) {
                unsafe {
                    *(send_target_i32.cast_mut()) = target.as_i32();
                }
            } else {
                log::error!("设置发送目标失败：无法获取发送目标地址");
            }
        }

        fn set_send_target_player(player_id: u64) {
            if let Some(player_u64) = utils::get_ptr_with_offset(
                game_export::U_GUI_CHAT_BASE as *const u64,
                game_export::U_GUI_CHAT_SEND_TARGET_PLAYER_OFFSETS,
            ) {
                unsafe {
                    *(player_u64.cast_mut()) = player_id;
                }
            } else {
                log::error!("设置发送目标玩家失败：无法获取发送目标玩家地址");
            }
        }
    }

    impl Default for ChatMessageSender {
        fn default() -> Self {
            Self::new()
        }
    }

    /// 接收聊天消息工具
    pub struct ChatMessageReceiver {
        prefix: String,
    }

    impl ChatMessageReceiver {
        pub fn new() -> Self {
            Self {
                prefix: String::new(),
            }
        }

        /// 前缀过滤
        ///
        /// 将会通过设置的前缀过滤消息\
        /// 只接收满足前缀的消息\
        /// 否则将会消费所有消息（这可能会导致其他进程无法接收消息）
        pub fn set_prefix_filter(&mut self, prefix: &str) {
            self.prefix = prefix.to_string();
        }

        pub fn try_recv(&self) -> Option<String> {
            let msg_ptr = utils::get_ptr_with_offset(
                game_export::MESSAGE_BASE,
                game_export::MESSAGE_BODY_OFFSETS,
            )?;
            let msg_ptr = msg_ptr as *mut u8;
            let msg_len_ptr = utils::get_ptr_with_offset(
                game_export::MESSAGE_BASE,
                game_export::MESSAGE_LEN_OFFSETS,
            )?;
            let msg_len = unsafe { *msg_len_ptr };
            if msg_len == 0 {
                return None;
            }
            let msg = unsafe {
                String::from_utf8_lossy(std::slice::from_raw_parts(msg_ptr, msg_len as usize))
            };
            if msg.starts_with(&self.prefix) {
                let msg = msg.to_string();
                // 清除缓冲区防止重复读
                unsafe {
                    std::ptr::write_bytes(msg_ptr, 0, msg_len as usize);
                    *(msg_len_ptr as *mut i32) = 0;
                }
                return Some(msg);
            }

            None
        }
    }

    impl Default for ChatMessageReceiver {
        fn default() -> Self {
            Self::new()
        }
    }
}

// 为了兼容性保留
pub use chat::ChatMessageReceiver;
pub use chat::ChatMessageSender;
