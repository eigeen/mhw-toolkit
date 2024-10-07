use crate::{
    game::prelude::{MtObject, Resource},
    game_export,
};

pub struct Quest {
    instance: usize,
}

impl MtObject for Quest {
    fn get_instance(&self) -> usize {
        self.instance
    }

    fn from_instance(ptr: usize) -> Self {
        Self { instance: ptr }
    }
}

impl Quest {
    pub fn new_static() -> Option<Self> {
        let ptr = unsafe { *(game_export::QUEST_BASE) };
        if ptr < 65536 {
            None
        } else {
            Some(Self { instance: ptr })
        }
    }

    pub fn quest_state(&self) -> i32 {
        self.get_value_copy(0x38)
    }

    pub fn quest_state_mut(&self) -> &mut i32 {
        self.get_value_mut(0x38)
    }

    pub fn quest_timer_max(&self) -> f32 {
        self.get_value_copy(0x13198 + 0x0C)
    }

    pub fn quest_timer_mut(&self) -> &mut f32 {
        self.get_value_mut(0x13198 + 0x08)
    }

    pub fn ensurance_state_mut(&self) -> &mut i8 {
        self.get_value_mut(0x17384)
    }
}
