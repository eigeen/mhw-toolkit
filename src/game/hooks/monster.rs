pub use ctor::hook_monster_ctor;
pub use dtor::hook_monster_dtor;

// ########## Create Monster ##########
mod ctor {
    use std::{
        ffi::c_void,
        ptr::{self, addr_of_mut},
        sync::atomic::{AtomicBool, Ordering},
    };

    use crate::game::hooks::{init_mh, HookError};

    type MonsterCtorFunction = extern "C" fn(*const c_void, i32, i32);
    static mut ORIGINAL_FUNCTION: *mut c_void = ptr::null_mut();
    static CTOR_HOOKED: AtomicBool = AtomicBool::new(false);
    static mut CTOR_CALLBACKS: Vec<Box<dyn Fn(usize)>> = Vec::new();

    extern "C" fn hooked_function(monster: *const c_void, type_id: i32, type_sub_id: i32) {
        // 调用钩子回调
        unsafe { CTOR_CALLBACKS.iter().for_each(|f| f(monster as usize)) }
        // 调用原始函数
        unsafe {
            let original: MonsterCtorFunction = std::mem::transmute(ORIGINAL_FUNCTION);
            original(monster, type_id, type_sub_id);
        }
    }

    /// 创建钩子
    ///
    /// 怪物生成时执行
    pub fn hook_monster_ctor<F>(f: F) -> Result<(), HookError>
    where
        F: Fn(usize) + 'static,
    {
        if !CTOR_HOOKED.load(Ordering::SeqCst) {
            create_ctor_hook()?;
            CTOR_HOOKED.store(true, Ordering::SeqCst)
        }
        unsafe { CTOR_CALLBACKS.push(Box::new(f)) }

        Ok(())
    }

    fn create_ctor_hook() -> Result<(), HookError> {
        unsafe {
            init_mh();

            let target_function: *mut c_void = 0x141CA1130 as *mut c_void;

            let create_hook_status = minhook_sys::MH_CreateHook(
                target_function,
                hooked_function as *mut c_void,
                addr_of_mut!(ORIGINAL_FUNCTION),
            );
            if create_hook_status != minhook_sys::MH_OK {
                return Err(HookError::CreateHook(create_hook_status));
            }

            let _ = minhook_sys::MH_EnableHook(target_function);
            let _ = minhook_sys::MH_ApplyQueued();

            Ok(())
        }
    }
}

// ########## Destroy Monster ##########
mod dtor {
    use std::{
        ffi::c_void,
        ptr::{self, addr_of_mut},
        sync::atomic::{AtomicBool, Ordering},
    };

    use crate::game::hooks::{init_mh, HookError};

    type MonsterDtorFunction = extern "C" fn(*const c_void);
    static mut ORIGINAL_FUNCTION: *mut c_void = ptr::null_mut();
    static DTOR_HOOKED: AtomicBool = AtomicBool::new(false);
    static mut DTOR_CALLBACKS: Vec<Box<dyn Fn(usize)>> = Vec::new();

    extern "C" fn hooked_function(monster: *const c_void) {
        // 调用钩子回调
        unsafe { DTOR_CALLBACKS.iter().for_each(|f| f(monster as usize)) }
        // 调用原始函数
        unsafe {
            let original: MonsterDtorFunction = std::mem::transmute(ORIGINAL_FUNCTION);
            original(monster);
        }
    }

    /// 创建钩子
    ///
    /// 怪物生成时执行
    pub fn hook_monster_dtor<F>(f: F) -> Result<(), HookError>
    where
        F: Fn(usize) + 'static,
    {
        if !DTOR_HOOKED.load(Ordering::SeqCst) {
            create_dtor_hook()?;
            DTOR_HOOKED.store(true, Ordering::SeqCst)
        }
        unsafe { DTOR_CALLBACKS.push(Box::new(f)) }

        Ok(())
    }

    fn create_dtor_hook() -> Result<(), HookError> {
        unsafe {
            init_mh();

            let target_function: *mut c_void = 0x141CA3A10 as *mut c_void;

            let create_hook_status = minhook_sys::MH_CreateHook(
                target_function,
                hooked_function as *mut c_void,
                addr_of_mut!(ORIGINAL_FUNCTION),
            );
            if create_hook_status != minhook_sys::MH_OK {
                return Err(HookError::CreateHook(create_hook_status));
            }

            let _ = minhook_sys::MH_EnableHook(target_function);
            let _ = minhook_sys::MH_ApplyQueued();

            Ok(())
        }
    }
}
