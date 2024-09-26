mod health;
mod monster;
mod player;

pub use health::*;
pub use monster::*;
pub use player::*;

use crate::game::prelude::{Model, Resource};

use super::ActionController;

pub trait Entity: Resource + Model {
    fn action_controller(&self) -> ActionController {
        self.get_inline_object(0x61C8)
    }
}
