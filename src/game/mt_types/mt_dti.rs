use super::{MtObject, Resource};

pub struct MtDti {
    instance: usize,
}

impl MtObject for MtDti {
    fn get_instance(&self) -> usize {
        self.instance
    }

    fn from_instance(ptr: usize) -> Self {
        Self { instance: ptr }
    }
}

impl MtDti {
    pub fn name(&self) -> String {
        todo!()
    }
}
