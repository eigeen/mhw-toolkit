use crate::game::{
    mt_types::{Model, MtObject, Resource},
    prelude::Entity,
};

const PLAYER_BASE: *const usize = 0x145011760 as *const _;
const PLAYER_OFFSET: &[isize] = &[0x50];

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
        let player_addr = crate::utils::get_value_with_offset(PLAYER_BASE, PLAYER_OFFSET)?;
        if player_addr == 0 {
            return None;
        }

        Some(Self::from_instance(player_addr))
    }

    pub fn set_frame_speed_multiplier(&self, multiplier: f32) {
        let addr = self.frame_speed_multiplier_addr();
        unsafe {
            *(addr as *mut f32) = multiplier;
        }
    }

    pub fn get_frame_speed_multiplier(&self) -> f32 {
        let addr = self.frame_speed_multiplier_addr();
        unsafe { *(addr as *const f32) }
    }

    fn frame_speed_multiplier_addr(&self) -> usize {
        unsafe {
            let a = *(0x145121688 as *const u32) as usize;
            let b = *((self.get_instance() + 0x10) as *const i32) as usize;
            let addr = a + b * 0xF8 + 0x9C;
            addr
        }
    }
}
