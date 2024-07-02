use std::ffi::CStr;

use crate::{
    game::{
        mt_types::{Model, MtObject, Resource},
        prelude::Entity,
    },
    utils,
};

const PLAYER_BASE: *const usize = 0x145011760 as *const _;
const PLAYER_OFFSET: &[isize] = &[0x50];
const PLAYER_NAME_OFFSET: &[isize] = &[0xC0, 0x8, 0x78, 0x78];

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

impl Resource for Player {}

impl Model for Player {}

impl Entity for Player {}

impl Player {
    pub fn current_player() -> Option<Self> {
        let player_addr = utils::get_value_with_offset(PLAYER_BASE, PLAYER_OFFSET)?;
        if player_addr == 0 {
            return None;
        }

        Some(Self::from_instance(player_addr))
    }

    pub fn get_frame_speed_multiplier_mut(&self) -> &'static mut f32 {
        let addr = self.frame_speed_multiplier_addr();
        unsafe { (addr as *mut f32).as_mut().unwrap() }
    }

    pub fn get_name(&self) -> &'static str {
        let name_ptr = match utils::get_ptr_with_offset(
            self.get_instance() as *const i8,
            PLAYER_NAME_OFFSET,
        ) {
            Some(p) => p,
            None => return "",
        };
        unsafe { CStr::from_ptr(name_ptr).to_str().unwrap_or_default() }
    }

    fn frame_speed_multiplier_addr(&self) -> usize {
        unsafe {
            let a = *(0x145121688 as *const u32) as usize;
            let b = *((self.get_instance() + 0x10) as *const i32) as usize;

            a + b * 0xF8 + 0x9C
        }
    }
}
