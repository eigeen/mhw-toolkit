use crate::game::mt_types::{MtObject, Resource};

// ########## ActionInfo ##########

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ActionInfo {
    pub set: i32,
    pub id: i32,
}

// ########## ActionController ##########

#[derive(Debug, Clone)]
pub struct ActionController {
    instance: usize,
}

impl MtObject for ActionController {
    fn get_instance(&self) -> usize {
        self.instance
    }

    fn from_instance(ptr: usize) -> Self {
        Self { instance: ptr }
    }
}

impl ActionController {
    pub fn current_action(&self) -> ActionInfo {
        self.get_value_copy(0xAC)
    }

    pub fn next_action(&self) -> ActionInfo {
        self.get_value_copy(0xBC)
    }

    pub fn previous_action(&self) -> ActionInfo {
        self.get_value_copy(0xC4)
    }

    pub fn force_derive(&self, action: ActionInfo) {
        let current_action = self.get_value_mut::<ActionInfo>(0xBC);
        current_action.set = action.set;
        current_action.id = action.id;
    }

    /// 控制器持有者（Entity）
    pub fn owner(&self) -> usize {
        self.get_value_copy(0x100)
    }
}
