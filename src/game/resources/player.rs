use crate::game::{mt_types::{Model, MtObject, Resource}, prelude::Entity};

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

        Some(Self::from_instance(player_addr))
    }
}
