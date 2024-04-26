use super::{Quaternion, Resource, Vec3};

pub trait Model: Resource {
    fn position(&self) -> &Vec3 {
        self.get_value_ref(0x160)
    }

    fn size(&self) -> &Vec3 {
        self.get_value_ref(0x180)
    }

    fn rotation(&self) -> &Quaternion {
        self.get_value_ref(0x170)
    }
}
