use super::MtDti;

/// Mt对象
///
/// 提供基础的实例指针操作
pub trait MtObject {
    fn get_instance(&self) -> usize;
    fn from_instance(ptr: usize) -> Self;
}

/// 资源对象
///
/// 提供资源内字段的访问操作
pub trait Resource: MtObject {
    /// 获得对象的成员的引用
    fn get_value_ref<T>(&self, offset: isize) -> &'static T {
        unsafe {
            let ptr: *const T = (self.get_instance() as isize + offset) as *const T;
            ptr.as_ref().unwrap()
        }
    }

    /// 获得对象的成员的可变引用
    fn get_value_mut<T>(&self, offset: isize) -> &'static mut T {
        unsafe {
            let ptr: *const T = (self.get_instance() as isize + offset) as *const T;
            ptr.cast_mut().as_mut().unwrap()
        }
    }

    /// 获得对象的成员的副本
    fn get_value_copy<T>(&self, offset: isize) -> T
    where
        T: Copy,
    {
        unsafe {
            let ptr = (self.get_instance() as isize + offset) as *const T;
            *ptr
        }
    }

    /// 获得对象的MtObject成员（指针指向的对象）
    fn get_object<T>(&self, offset: isize) -> T
    where
        T: MtObject,
    {
        unsafe {
            let ptr = (self.get_instance() as isize + offset) as *const *const T;
            T::from_instance(*ptr as usize)
        }
    }

    /// 获得对象的MtObject成员（inline对象）
    fn get_inline_object<T>(&self, offset: isize) -> T
    where
        T: MtObject,
    {
        let ptr = self.get_instance() as isize + offset;
        T::from_instance(ptr as usize)
    }

    /// 获取对象的虚函数
    unsafe fn get_virtual_function(&self, index: isize) -> usize {
        let vtable = *(self.get_instance() as *const *const usize);
        let vfptr = vtable.offset(index);

        vfptr as usize
    }

    unsafe fn get_dti(&self) -> Option<MtDti> {
        todo!()
    }
}



#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn distance_of(&self, other: &Self) -> f32 {
        ((self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z))
            .sqrt()
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}
