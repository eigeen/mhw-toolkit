use crate::game::mt_array::{MtArray, MtObject};

use super::Resource;

pub struct ObjCollision {
    instance: usize,
}

impl Resource for ObjCollision {
    fn get_instance(&self) -> *mut Self {
        self.instance as *mut Self
    }
}

impl ObjCollision {
    pub fn new(ptr: usize) -> Self {
        Self { instance: ptr }
    }

    pub fn coll_node_resource(&self) -> CollNodeResource {
        self.get_object(0xB0)
    }
}

pub struct CollNodeResource {
    instance: usize,
}

impl Resource for CollNodeResource {
    fn get_instance(&self) -> *mut Self {
        self.instance as *mut Self
    }
}

impl CollNodeResource {
    pub fn new(ptr: usize) -> Self {
        Self { instance: ptr }
    }

    pub fn node_type(&self) -> CollNodeType {
        self.get_object(0xA8)
    }

    pub fn nodes(&self) -> MtArray<CollNode> {
        MtArray::new(self.get_instance() as usize + 0xB0)
    }
}

pub struct CollNode {
    instance: usize,
}

impl Resource for CollNode {
    fn get_instance(&self) -> *mut Self {
        self.instance as *mut Self
    }
}

impl MtObject for CollNode {
    fn from_instance(ptr: usize) -> Self {
        Self::new(ptr)
    }
}

impl CollNode {
    pub fn new(ptr: usize) -> Self {
        Self { instance: ptr }
    }

    pub fn geometry(&self) -> &CollGeomResource {
        self.get_object(0x8)
    }
}

pub struct CollGeomResource {
    instance: usize,
}

impl Resource for CollGeomResource {
    fn get_instance(&self) -> *mut Self {
        self.instance as *mut Self
    }
}

impl CollGeomResource {
    pub fn new(ptr: usize) -> Self {
        Self { instance: ptr }
    }
}


#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CollNodeType {
    Default = 0,
    Player = 1,
    Monster = 2,
    Player3 = 3,
    Default4 = 4,
    Default5 = 5,
    Default6 = 6,
}
