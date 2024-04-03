use crate::util::{get_value_with_offset, RawPtr};

pub const XBOX_PAD_PTR: *const f32 = 0x1451C2318 as *const f32;

pub const PLAYER_PTR: *const i32 = 0x145011760 as *const i32;
pub const PLAYER_OFFSET: isize = 0x50;
pub const PLAYER_DATA_PTR: *const i32 = 0x145011760 as *const i32;
pub const PLAYER_DATA_OFFSET: &[isize] = &[0x50, 0xC0, 0x98, 0x18, 0x70, 0xC8, 0xD0, 0x5D0, 0x20];

pub const CHAT_MAIN_PTR: *const usize = 0x14500AC30 as *const usize;
pub const U_GUI_CHAT_BASE: *const usize = 0x1451C2400 as *const usize;
pub const U_GUI_CHAT_STRUCT_OFFSETS: &[isize] = &[0x13FD0, 0x28F8];
pub const U_GUI_CHAT_SEND_OFFSETS: &[isize] = &[0x13FD0, 0x325E];

pub const MESSAGE_BASE: *const i32 = 0x144F85DB0 as *const i32;
pub const MESSAGE_LEN_OFFSETS: &[isize] = &[0xBC];
pub const MESSAGE_BODY_OFFSETS: &[isize] = &[0xC0];

/// 怪物数据
pub struct Monster {}

/// 玩家数据
/// Position                坐标信息
/// Model                   模型信息
/// Collimator              准星信息
/// Angle                   角度信息
/// Weapon                  武器信息
/// Equip                   装备信息
/// Characteristic          属性信息
/// Gravity                 重力信息
pub struct Player {
    player: RawPtr<i32>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            player: RawPtr::new(PLAYER_PTR).offset(PLAYER_OFFSET),
        }
    }

    #[inline]
    fn get_player(&mut self) -> Option<*const i32> {
        self.player.get_ptr()
    }

    /// 动作信息
    pub fn get_action(&mut self) -> Option<PlayerAction> {
        if let Some(player) = self.get_player() {
            Some(PlayerAction {
                lmt_id: get_value_with_offset(player, &[0x468, 0xE9C4]).unwrap_or_default(),
                fsm: FSM {
                    fsm_target: unsafe { *player.byte_offset(0x6274) },
                    fsm_id: unsafe { *player.byte_offset(0x6278) },
                },
                use_item: unsafe { *player.byte_offset(0xb780) },
            })
        } else {
            None
        }
    }

    /// 动作帧信息
    pub fn get_frame(&mut self) -> Option<PlayerFrame> {
        if let Some(player) = self.get_player() {
            let multiplier_offset = unsafe { *player.byte_offset(0x10) } * 0xf8 + 0x9c;
            Some(PlayerFrame {
                frame: get_value_with_offset(player as *const f32, &[0x468, 0x10C])
                    .unwrap_or_default(),
                frame_end: get_value_with_offset(player as *const f32, &[0x468, 0x114])
                    .unwrap_or_default(),
                frame_speed: unsafe { *(player as *const f32).byte_offset(0x6c) },
                frame_speed_multiplier: unsafe {
                    *(0x145121688 as *const f32).byte_offset(multiplier_offset as isize)
                },
            })
        } else {
            None
        }
    }

    pub fn set(&mut self, field: PlayerField) {
        unimplemented!();
        match field {
            PlayerField::Action(action) => match action {
                ActionField::LMTID(v) => todo!(),
                ActionField::FSM(fsm) => match fsm {
                    FSMField::Target(v) => todo!(),
                    FSMField::ID(v) => todo!(),
                },
                ActionField::UseItem(v) => todo!(),
            },
            PlayerField::Frame(frame) => match frame {
                FrameField::Frame(v) => todo!(),
                FrameField::FrameEnd(v) => todo!(),
                FrameField::FrameSpeed(v) => todo!(),
                FrameField::FrameSpeedMultiplier(v) => todo!(),
            },
        };
    }
}

pub enum PlayerField {
    Action(ActionField),
    Frame(FrameField),
}

pub enum ActionField {
    LMTID(i32),
    FSM(FSMField),
    UseItem(i32),
}

pub enum FSMField {
    Target(i32),
    ID(i32),
}

pub enum FrameField {
    Frame(f32),
    FrameEnd(f32),
    FrameSpeed(f32),
    FrameSpeedMultiplier(f32),
}

#[derive(Default)]
pub struct PlayerAction {
    lmt_id: i32,
    fsm: FSM,
    use_item: i32,
}

#[derive(Default)]
pub struct FSM {
    fsm_target: i32,
    fsm_id: i32,
}

#[derive(Default)]
pub struct PlayerFrame {
    frame: f32,
    frame_end: f32,
    frame_speed: f32,
    frame_speed_multiplier: f32,
}
