pub use ctor::MonsterCtorHook;
pub use dtor::MonsterDtorHook;

// ########## Create Monster ##########
mod ctor {
    use std::{
        collections::HashMap,
        ffi::c_void,
        ptr::{self, addr_of_mut},
        sync::{
            atomic::{AtomicBool, Ordering},
            Mutex,
        },
    };

    use once_cell::sync::Lazy;
    use rand::RngCore;

    use crate::game::hooks::{init_mh, CallbackPosition, HookError, HookHandle};

    type MonsterCtorFunction = extern "C" fn(*const c_void, i32, i32);
    type Args = (*const c_void, i32, i32);
    type CallbackFn = Box<dyn Fn(Args) + 'static + Send + Sync>;
    type CallbacksTable = HashMap<CallbackPosition, Vec<(u64, CallbackFn)>>;

    static mut ORIGINAL_FUNCTION: *mut c_void = ptr::null_mut();
    static CTOR_HOOKED: AtomicBool = AtomicBool::new(false);
    static CTOR_CALLBACKS: Lazy<Mutex<CallbacksTable>> = Lazy::new(|| Mutex::new(HashMap::new()));

    extern "C" fn hooked_function(monster: *const c_void, type_id: i32, type_sub_id: i32) {
        // Before
        if let Some(callbacks) = CTOR_CALLBACKS
            .lock()
            .unwrap()
            .get(&CallbackPosition::Before)
        {
            callbacks
                .iter()
                .for_each(|(_, f)| f((monster, type_id, type_sub_id)))
        }
        // 调用原始函数
        unsafe {
            let original: MonsterCtorFunction = std::mem::transmute(ORIGINAL_FUNCTION);
            original(monster, type_id, type_sub_id);
        }
        // After
        if let Some(callbacks) = CTOR_CALLBACKS.lock().unwrap().get(&CallbackPosition::After) {
            callbacks
                .iter()
                .for_each(|(_, f)| f((monster, type_id, type_sub_id)))
        }
    }

    fn hook_once() -> Result<(), HookError> {
        if !CTOR_HOOKED.load(Ordering::SeqCst) {
            create_ctor_hook()?;
            CTOR_HOOKED.store(true, Ordering::SeqCst)
        }
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

    pub struct MonsterCtorHook {
        inner_id: u64,
        position: Option<CallbackPosition>,
    }

    impl HookHandle for MonsterCtorHook {
        type Args = Args;

        fn set_hook<F>(&mut self, position: CallbackPosition, f: F) -> Result<(), HookError>
        where
            F: Fn(Self::Args) + 'static + Send + Sync,
        {
            hook_once()?;
            self.position = Some(position);
            CTOR_CALLBACKS
                .lock()
                .unwrap()
                .entry(position)
                .or_default()
                .push((self.inner_id, Box::new(f)));
            Ok(())
        }

        fn unset_hook(&mut self) -> Result<(), HookError> {
            if self.position.is_none() {
                return Err(HookError::HookNotSet);
            }

            let mut callbacks = CTOR_CALLBACKS.lock().unwrap();
            let callbacks = callbacks
                .get_mut(&self.position.unwrap())
                .ok_or(HookError::HookNotSet)?;
            let pos = callbacks
                .iter()
                .position(|(id, _)| *id == self.inner_id)
                .ok_or(HookError::HookNotSet)?;
            let _ = callbacks.remove(pos);
            self.position = None;
            Ok(())
        }

        fn is_hooked(&self) -> bool {
            self.position.is_some()
        }
    }

    impl Default for MonsterCtorHook {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Drop for MonsterCtorHook {
        fn drop(&mut self) {
            let _ = self.unset_hook();
        }
    }

    impl MonsterCtorHook {
        pub fn new() -> Self {
            Self {
                inner_id: rand::thread_rng().next_u64(),
                position: None,
            }
        }
    }
}

// ########## Destroy Monster ##########
mod dtor {
    use std::{
        collections::HashMap,
        ffi::c_void,
        ptr::{self, addr_of_mut},
        sync::{
            atomic::{AtomicBool, Ordering},
            Mutex,
        },
    };

    use once_cell::sync::Lazy;
    use rand::RngCore;

    use crate::game::hooks::{init_mh, CallbackPosition, HookError, HookHandle};

    type MonsterDtorFunction = extern "C" fn(*const c_void);
    type Args = *const c_void;
    type CallbackFn = Box<dyn Fn(Args) + 'static + Send + Sync>;
    type CallbacksTable = HashMap<CallbackPosition, Vec<(u64, CallbackFn)>>;

    static mut ORIGINAL_FUNCTION: *mut c_void = ptr::null_mut();
    static DTOR_HOOKED: AtomicBool = AtomicBool::new(false);
    static DTOR_CALLBACKS: Lazy<Mutex<CallbacksTable>> = Lazy::new(|| Mutex::new(HashMap::new()));

    extern "C" fn hooked_function(monster: *const c_void) {
        // Before
        if let Some(callbacks) = DTOR_CALLBACKS
            .lock()
            .unwrap()
            .get(&CallbackPosition::Before)
        {
            callbacks.iter().for_each(|(_, f)| f(monster))
        }
        // 调用原始函数
        unsafe {
            let original: MonsterDtorFunction = std::mem::transmute(ORIGINAL_FUNCTION);
            original(monster);
        }
        // After
        if let Some(callbacks) = DTOR_CALLBACKS.lock().unwrap().get(&CallbackPosition::After) {
            callbacks.iter().for_each(|(_, f)| f(monster))
        }
    }

    fn hook_once() -> Result<(), HookError> {
        if !DTOR_HOOKED.load(Ordering::SeqCst) {
            create_dtor_hook()?;
            DTOR_HOOKED.store(true, Ordering::SeqCst)
        }
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

    pub struct MonsterDtorHook {
        inner_id: u64,
        position: Option<CallbackPosition>,
    }

    impl HookHandle for MonsterDtorHook {
        type Args = Args;

        fn set_hook<F>(&mut self, position: CallbackPosition, f: F) -> Result<(), HookError>
        where
            F: Fn(Self::Args) + 'static + Send + Sync,
        {
            hook_once()?;
            self.position = Some(position);
            DTOR_CALLBACKS
                .lock()
                .unwrap()
                .entry(position)
                .or_default()
                .push((self.inner_id, Box::new(f)));
            Ok(())
        }

        fn unset_hook(&mut self) -> Result<(), HookError> {
            if self.position.is_none() {
                return Err(HookError::HookNotSet);
            }

            let mut callbacks = DTOR_CALLBACKS.lock().unwrap();
            let callbacks = callbacks
                .get_mut(&self.position.unwrap())
                .ok_or(HookError::HookNotSet)?;
            let pos = callbacks
                .iter()
                .position(|(id, _)| *id == self.inner_id)
                .ok_or(HookError::HookNotSet)?;
            let _ = callbacks.remove(pos);
            self.position = None;
            Ok(())
        }

        fn is_hooked(&self) -> bool {
            self.position.is_some()
        }
    }

    impl Default for MonsterDtorHook {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Drop for MonsterDtorHook {
        fn drop(&mut self) {
            let _ = self.unset_hook();
        }
    }

    impl MonsterDtorHook {
        pub fn new() -> Self {
            Self {
                inner_id: rand::thread_rng().next_u64(),
                position: None,
            }
        }
    }
}
