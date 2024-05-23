use std::{
    ffi::c_void,
    ptr::{self, addr_of_mut},
    sync::atomic::{AtomicBool, Ordering},
};

use crate::game::{
    mt_types::MtObject,
    resources::{ActionController, ActionInfo},
};

use super::{init_mh, HookError};

type DoActionFunction = extern "C" fn(*const c_void, *const ActionInfo);
type Callback = dyn Fn(ActionController, &mut ActionInfo);

static mut ORIGINAL_FUNCTION: *mut c_void = ptr::null_mut();
static HOOKED: AtomicBool = AtomicBool::new(false);
static mut HOOK_CALLBACKS: Vec<Box<Callback>> = Vec::new();

extern "C" fn hooked_function(controller: *const c_void, action_info: *mut ActionInfo) {
    // 调用钩子回调
    unsafe {
        let controller = ActionController::from_instance(controller as usize);
        let action_info = &mut *action_info;
        HOOK_CALLBACKS
            .iter()
            .for_each(|f| f(controller.clone(), action_info))
    }
    // 调用原始函数
    unsafe {
        let original: DoActionFunction = std::mem::transmute(ORIGINAL_FUNCTION);
        original(controller, action_info);
    }
}

/// 创建钩子
///
/// ActionController.DoAction
pub fn hook_action<F>(f: F) -> Result<(), HookError>
where
    F: Fn(ActionController, &mut ActionInfo) + 'static,
{
    if !HOOKED.load(Ordering::SeqCst) {
        create_hook()?;
        HOOKED.store(true, Ordering::SeqCst)
    }
    unsafe { HOOK_CALLBACKS.push(Box::new(f)) }

    Ok(())
}

fn create_hook() -> Result<(), HookError> {
    unsafe {
        init_mh();

        let target_function: *mut c_void = 0x140269C90 as *mut c_void;

        // 创建钩子
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
