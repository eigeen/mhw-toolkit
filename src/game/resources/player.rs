use crate::game::mt_types::{Model, MtObject, Resource};

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

impl Player {}
