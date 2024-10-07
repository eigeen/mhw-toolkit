use crate::{
    game::prelude::{MtObject, Resource},
    utils,
};

const SAVE_BASE: *const usize = 0x145011710 as *const usize;

pub struct SaveData {
    instance: usize,
    save_offset: usize,
}

impl MtObject for SaveData {
    fn get_instance(&self) -> usize {
        self.instance + self.save_offset
    }

    fn from_instance(ptr: usize) -> Self {
        Self {
            instance: ptr,
            save_offset: 0,
        }
    }
}

impl SaveData {
    pub fn from_index(index: i32) -> Option<Self> {
        if !(0..=2).contains(&index) {
            return None;
        }
        let instance = utils::get_value_with_offset(SAVE_BASE, &[0xA8])?;
        let save_offset = index as usize * 0x26CC00;

        Some(Self {
            instance,
            save_offset,
        })
    }

    pub fn current_save() -> Option<Self> {
        let save_slot = Self::current_save_slot()?;
        Self::from_index(save_slot)
    }

    fn current_save_slot() -> Option<i32> {
        utils::get_value_with_offset(SAVE_BASE as *const i32, &[0xA0])
    }
}
