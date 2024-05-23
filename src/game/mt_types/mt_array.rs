use std::marker::PhantomData;

use super::{MtObject, Resource};

pub struct MtArray<T> {
    instance: usize,
    _obj: PhantomData<T>,
}

impl<T> MtObject for MtArray<T>
where
    T: MtObject,
{
    fn get_instance(&self) -> usize {
        self.instance
    }

    fn from_instance(ptr: usize) -> Self {
        Self {
            instance: ptr,
            _obj: PhantomData,
        }
    }
}

impl<T> Resource for MtArray<T> where T: MtObject {}

impl<T> MtArray<T>
where
    T: MtObject,
{
    pub fn new(ptr: usize) -> MtArray<T> {
        MtArray {
            instance: ptr,
            _obj: PhantomData,
        }
    }

    pub fn length(&self) -> u32 {
        self.get_value_copy(0x8)
    }

    pub fn capacity(&self) -> u32 {
        self.get_value_copy(0xC)
    }

    pub fn auto_delete(&self) -> bool {
        self.get_value_copy(0x10)
    }

    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }

    /// 数据域
    unsafe fn data(&self) -> *const T {
        *(self.get_value_copy::<usize>(0x18) as *const *const T)
    }

    pub fn object_at(&self, index: isize) -> T {
        unsafe {
            let ptr = self.data().offset(index) as usize;

            MtObject::from_instance(ptr)
        }
    }
}
