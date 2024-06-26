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

use crate::game::{
    address::{self, AddressRepository},
    prelude::MtObject,
    resources::{ActionController, ActionInfo},
};

use super::{init_mh, CallbackPosition, HookError, HookHandle};

type DoActionFunction = extern "C" fn(*const c_void, *const ActionInfo) -> i8;
type Args = (ActionController, &'static mut ActionInfo);
type CallbackFn = Box<dyn Fn(Args) + 'static + Send + Sync>;
type CallbacksTable = HashMap<CallbackPosition, Vec<(u64, CallbackFn)>>;

static mut ORIGINAL_FUNCTION: *mut c_void = ptr::null_mut();
static HOOKED: AtomicBool = AtomicBool::new(false);
static HOOK_CALLBACKS: Lazy<Mutex<CallbacksTable>> = Lazy::new(|| Mutex::new(HashMap::new()));
static SKIP_CALL: AtomicBool = AtomicBool::new(false);

extern "C" fn hooked_function(controller: *const c_void, action_info: *mut ActionInfo) -> i8 {
    // Before
    if let Some(callbacks) = HOOK_CALLBACKS
        .lock()
        .unwrap()
        .get(&CallbackPosition::Before)
    {
        callbacks.iter().for_each(|(_, f)| {
            let arg1 = ActionController::from_instance(controller as usize);
            let arg2 = unsafe { &mut *action_info };
            f((arg1, arg2))
        })
    }
    // 调用原始函数
    unsafe {
        let original: DoActionFunction = std::mem::transmute(ORIGINAL_FUNCTION);
        original(controller, action_info)
    }
}

fn hook_once() -> Result<(), HookError> {
    if !HOOKED.load(Ordering::SeqCst) {
        create_do_action_hook()?;
        HOOKED.store(true, Ordering::SeqCst)
    }
    Ok(())
}

fn create_do_action_hook() -> Result<(), HookError> {
    unsafe {
        init_mh();

        // 目标函数
        let func_addr = AddressRepository::get_instance()
            .lock()
            .unwrap()
            .get_address(address::action::SetAction)
            .map_err(HookError::CannotFindAddress)?;
        let target_function: *mut c_void = func_addr as *mut c_void;

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

pub struct DoActionHook {
    inner_id: u64,
    position: Option<CallbackPosition>,
}

impl HookHandle for DoActionHook {
    type Args = Args;

    fn set_hook<F>(&mut self, position: CallbackPosition, f: F) -> Result<(), HookError>
    where
        F: Fn(Self::Args) + 'static + Send + Sync,
    {
        hook_once()?;
        if position == CallbackPosition::After {
            return Err(HookError::UnsupportedPosition);
        }
        self.position = Some(position);
        HOOK_CALLBACKS
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

        let mut callbacks = HOOK_CALLBACKS.lock().unwrap();
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

    fn skip_call(&self, skip: bool) -> bool {
        if hook_once().is_err() {
            return false;
        };
        SKIP_CALL
            .compare_exchange(!skip, skip, Ordering::SeqCst, Ordering::SeqCst)
            .map(|_| true)
            .unwrap_or(false)
    }
}

impl Default for DoActionHook {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DoActionHook {
    fn drop(&mut self) {
        let _ = self.unset_hook();
    }
}

impl DoActionHook {
    pub fn new() -> Self {
        Self {
            inner_id: rand::thread_rng().next_u64(),
            position: None,
        }
    }
}
