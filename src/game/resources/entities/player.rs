use std::ffi::CStr;

use crate::{
    game::mt_types::{Model, MtObject, Resource},
    game_export, utils,
};

use super::{Entity, Health};

// ##### Player 玩家对象 #####

/// 玩家对象
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Player {
    instance: usize,
}

impl MtObject for Player {
    fn get_instance(&self) -> usize {
        self.instance
    }

    fn from_instance(ptr: usize) -> Self {
        Self { instance: ptr }
    }
}

impl Model for Player {}

impl Entity for Player {}

impl std::fmt::Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player")
            .field("instance", &format!("0x{:X}", self.instance))
            .finish()
    }
}

impl Player {
    /// 获取当前操控的玩家对象
    pub fn current_player() -> Option<Self> {
        let player_addr =
            utils::get_value_with_offset(game_export::PLAYER_BASE, game_export::PLAYER_OFFSET)?;
        if player_addr == 0 {
            return None;
        }

        Some(Self::from_instance(player_addr))
    }

    pub fn from_index(index: isize) -> Option<Self> {
        if !(0..20).contains(&index) {
            return None;
        }
        if index == 3 {
            return Self::current_player();
        }
        let offset = 0x58 + 0x740 * index;
        let player_addr = utils::get_value_with_offset(game_export::PLAYER_BASE, &[offset])?;
        if !(0x10000..0x150000000).contains(&player_addr) && player_addr != u32::MAX as usize {
            return None;
        }

        Some(Self::from_instance(player_addr))
    }

    pub fn frame_speed_multiplier_mut(&self) -> &'static mut f32 {
        let addr = self.frame_speed_multiplier_addr();
        unsafe { (addr as *mut f32).as_mut().unwrap() }
    }

    pub fn info(&self) -> Option<PlayerInfo> {
        let info_addr = utils::get_value_with_offset(
            (self.get_instance() + 0xC0) as *const usize,
            &[0x8, 0x78],
        )?;

        Some(PlayerInfo::from_instance(info_addr))
    }

    pub fn short_info(&self) -> Option<PlayerShortInfo> {
        let info = self.info()?;
        info.short_info()
    }

    pub fn weapon_info(&self) -> Option<PlayerWeaponInfo> {
        if self.get_value_copy::<usize>(0x76B0) == 0 {
            return None;
        }

        Some(self.get_inline_object(0x76B0))
    }

    pub fn health(&self) -> Health {
        self.get_object(0x7630)
    }

    pub fn quest_id(&self) -> i32 {
        self.get_value_copy(0x10D8)
    }

    fn frame_speed_multiplier_addr(&self) -> usize {
        unsafe {
            let a = *(game_export::PLAYER_FRAME_SPEED_BASE) as usize;
            let b = *((self.get_instance() + 0x10) as *const i32) as usize;

            a + b * 0xF8 + 0x9C
        }
    }
}

// ##### PlayerInfo 玩家详细信息 #####

/// 玩家详细信息
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerInfo {
    instance: usize,
}

impl MtObject for PlayerInfo {
    fn get_instance(&self) -> usize {
        self.instance
    }

    fn from_instance(ptr: usize) -> Self {
        Self { instance: ptr }
    }
}

impl PlayerInfo {
    pub fn from_index(index: isize) -> Option<Self> {
        if !(0..20).contains(&index) {
            return None;
        }

        let offset = 0x58 + 0x740 * index;
        let info_ptr = utils::get_ptr_with_offset(game_export::PLAYER_BASE, &[offset])?;
        if !(0x10000..0x150000000).contains(&(info_ptr as usize)) {
            return None;
        }

        let this = Self::from_instance(info_ptr as usize);
        if this.name().is_empty() {
            None
        } else {
            Some(this)
        }
    }

    pub fn short_info(&self) -> Option<PlayerShortInfo> {
        let name = self.name();
        if name.is_empty() {
            return None;
        }

        PlayerShortInfo::from_name(name)
    }

    pub fn name(&self) -> &'static str {
        let name_ptr = (self.get_instance() as *const i8).wrapping_byte_add(0x78);

        unsafe { CStr::from_ptr(name_ptr).to_str().unwrap_or_default() }
    }

    pub fn steam_id(&self) -> u64 {
        self.get_value_copy(0xE8)
    }
}

// ##### PlayerShortInfo 玩家简略信息 #####

/// 玩家简略信息
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerShortInfo {
    instance: usize,
}

impl MtObject for PlayerShortInfo {
    fn get_instance(&self) -> usize {
        self.instance
    }

    fn from_instance(ptr: usize) -> Self {
        Self { instance: ptr }
    }
}

impl PlayerShortInfo {
    pub fn from_index(index: isize) -> Option<Self> {
        if !(0..20).contains(&index) {
            return None;
        }
        let offset = index * 0x58;
        let ptr =
            utils::get_value_with_offset(game_export::PLAYER_SHORT_INFO_BASE, &[0x1AB0 + offset])?;
        if ptr == 0 {
            return None;
        }
        let this = Self::from_instance(ptr as usize);
        if this.name().is_empty() {
            None
        } else {
            Some(this)
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        for i in 0..28 {
            if let Some(short_info) = Self::from_index(i) {
                if short_info.name() == name {
                    return Some(short_info);
                }
            }
        }

        None
    }

    pub fn name(&self) -> &'static str {
        let name_ptr = (self.get_instance() as *const i8).wrapping_byte_add(0x49);

        unsafe { CStr::from_ptr(name_ptr).to_str().unwrap_or_default() }
    }

    pub fn level(&self) -> ShortLevelInfo {
        self.get_value_copy(0x70)
    }

    pub fn weapon(&self) -> WeaponInfo {
        WeaponInfo {
            r#type: self.get_value_copy(0x7C),
            id: self.get_value_copy(0x74),
        }
    }

    /// 玩家状况：所在区域/等待出发/任务/参加救援等
    pub fn status(&self) -> PlayerStatus {
        let basic_status: i8 = self.get_value_copy(0x86);

        if basic_status == 7 {
            let is_rescue: bool = self.get_value_copy(0x84);
            if is_rescue {
                return PlayerStatus::Rescue;
            } else {
                return PlayerStatus::InQuest;
            }
        }
        // 4: 集会区域(月辰)

        PlayerStatus::Undefined
    }
}

// ##### PlayerWeaponInfo 玩家武器信息 #####

/// 玩家武器信息
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerWeaponInfo {
    instance: usize,
}

impl MtObject for PlayerWeaponInfo {
    fn get_instance(&self) -> usize {
        self.instance
    }

    fn from_instance(ptr: usize) -> Self {
        Self { instance: ptr }
    }
}

impl PlayerWeaponInfo {
    pub fn weapon(&self) -> WeaponInfo {
        WeaponInfo {
            r#type: self.get_value_copy(0x9F8),
            id: self.get_value_copy(0x9FC),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShortLevelInfo {
    pub hr: i16,
    pub mr: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WeaponInfo {
    pub r#type: i32,
    pub id: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerStatus {
    /// 未定义
    Undefined,
    /// 任务
    InQuest,
    /// 参加救援
    Rescue,
}
