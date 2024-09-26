use crate::game::prelude::{MtObject, Resource};

pub struct Health {
    instance: usize,
}

impl MtObject for Health {
    fn get_instance(&self) -> usize {
        self.instance
    }

    fn from_instance(ptr: usize) -> Self {
        Self { instance: ptr }
    }
}

impl Resource for Health {}

impl Health {
    pub fn max(&self) -> f32 {
        self.get_value_copy(0x60)
    }

    pub fn current(&self) -> f32 {
        self.get_value_copy(0x64)
    }

    pub fn max_mut(&self) -> &mut f32 {
        self.get_value_mut(0x60)
    }

    pub fn current_mut(&self) -> &mut f32 {
        self.get_value_mut(0x64)
    }
}
