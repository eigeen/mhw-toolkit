// updated to 15.23.00
pub const XBOX_PAD_PTR: *const f32 = 0x1451C4558 as *const f32; // 0x1451C2318 15.22

pub const PLAYER_BASE: *const usize = 0x1450139A0 as *const usize; // 0x145011760 15.22
pub const PLAYER_OFFSET: &[isize] = &[0x50];
pub const PLAYER_DATA_PTR: *const i32 = 0x1450139A0 as *const i32; // 0x145011760 15.22
pub const PLAYER_DATA_OFFSET: &[isize] = &[0x50, 0xC0, 0x98, 0x18, 0x70, 0xC8, 0xD0, 0x5D0, 0x20];

pub const QUEST_BASE: *const usize = 0x14500ED30 as *const usize; // 0x14500CAF0 15.22

pub const SESSION_BASE: *const i32 = 0x1451C46B8 as *const i32;
pub const SESSION_PARTY_SIZE_OFFSETS: &[isize] = &[0x258, 0x10, 0x6574];

pub const CHAT_MAIN_PTR: *const usize = 0x14500CE70 as *const usize; // 0x14500AC30 15.22

pub const PLAYER_SHORT_INFO_BASE: *const usize = 0x145013530 as *const _;
pub const PLAYER_FRAME_SPEED_BASE: *const u32 = 0x1451238C8 as *const _;

pub const U_GUI_CHAT_BASE: *const usize = 0x1451C4640 as *const usize; // 0x1451C2400 15.22
pub const U_GUI_CHAT_STRUCT_OFFSETS: &[isize] = &[0x13FD0, 0x28F8];
pub const U_GUI_CHAT_SEND_OFFSETS: &[isize] = &[0x13FD0, 0x325E];
pub const U_GUI_CHAT_SEND_TARGET_OFFSETS: &[isize] = &[0x14748];
pub const U_GUI_CHAT_SEND_TARGET_PLAYER_OFFSETS: &[isize] = &[0x14748 + 0x8];

pub const MESSAGE_BASE: *const i32 = 0x144F87FF0 as *const i32; // 0x144F85DB0 15.22
pub const MESSAGE_LEN_OFFSETS: &[isize] = &[0xBC];
pub const MESSAGE_BODY_OFFSETS: &[isize] = &[0xC0];
