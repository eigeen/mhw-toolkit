use crate::game::resources::ActionController;

use super::{Model, Resource};

pub trait Entity: Resource + Model {
    fn action_controller(&self) -> ActionController {
        self.get_inline_object(0x61C8)
    }
}
