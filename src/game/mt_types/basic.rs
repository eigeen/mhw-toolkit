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
            let ptr: *const T = std::mem::transmute(self.get_instance() as isize + offset);
            ptr.as_ref().unwrap()
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
            MtObject::from_instance(*ptr as usize)
        }
    }

    /// 获得对象的MtObject成员（inline对象）
    fn get_inline_object<T>(&self, offset: isize) -> T
    where
        T: MtObject,
    {
        let ptr = self.get_instance() as isize + offset;
        MtObject::from_instance(ptr as usize)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}
