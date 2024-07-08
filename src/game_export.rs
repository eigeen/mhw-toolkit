pub const XBOX_PAD_PTR: *const f32 = 0x1451C2318 as *const f32;

pub const PLAYER_PTR: *const i32 = 0x145011760 as *const i32;
pub const PLAYER_OFFSET: isize = 0x50;
pub const PLAYER_DATA_PTR: *const i32 = 0x145011760 as *const i32;
pub const PLAYER_DATA_OFFSET: &[isize] = &[0x50, 0xC0, 0x98, 0x18, 0x70, 0xC8, 0xD0, 0x5D0, 0x20];

pub const CHAT_MAIN_PTR: *const usize = 0x14500AC30 as *const usize;
pub const U_GUI_CHAT_BASE: *const usize = 0x1451C2400 as *const usize;
pub const U_GUI_CHAT_STRUCT_OFFSETS: &[isize] = &[0x13FD0, 0x28F8];
pub const U_GUI_CHAT_SEND_OFFSETS: &[isize] = &[0x13FD0, 0x325E];
pub const U_GUI_CHAT_SEND_TARGET_OFFSETS: &[isize] = &[0x14748];

pub const MESSAGE_BASE: *const i32 = 0x144F85DB0 as *const i32;
pub const MESSAGE_LEN_OFFSETS: &[isize] = &[0xBC];
pub const MESSAGE_BODY_OFFSETS: &[isize] = &[0xC0];
