use std::{
    collections::HashMap,
    ffi::{c_void, CStr},
    ptr::{self, addr_of_mut},
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
};

use once_cell::sync::Lazy;
use rand::RngCore;

use crate::game::address::{self, AddressRepository};

use super::{init_mh, CallbackPosition, HookError, HookHandle};

type InputDispatchFunction = extern "C" fn(*const i8) -> i8;
type Args = &'static str;
type CallbackFn = Box<dyn Fn(Args) + 'static + Send + Sync>;
type CallbacksTable = HashMap<CallbackPosition, Vec<(u64, CallbackFn)>>;

static mut ORIGINAL_FUNCTION: *mut c_void = ptr::null_mut();
static HOOKED: AtomicBool = AtomicBool::new(false);
static HOOK_CALLBACKS: Lazy<Mutex<CallbacksTable>> = Lazy::new(|| Mutex::new(HashMap::new()));

extern "C" fn hooked_function(a1: *const i8) -> i8 {
    let inputs_ptr = unsafe { a1.byte_offset(0x1008) };
    let input_cstr = unsafe { CStr::from_ptr(inputs_ptr) };
    let input_str = input_cstr.to_str().unwrap_or_default();

    // Before
    if let Some(callbacks) = HOOK_CALLBACKS
        .lock()
        .unwrap()
        .get(&CallbackPosition::Before)
    {
        callbacks.iter().for_each(|(_, f)| f(input_str))
    }
    // 调用原始函数
    unsafe {
        let original: InputDispatchFunction = std::mem::transmute(ORIGINAL_FUNCTION);
        original(inputs_ptr)
    }
}

fn hook_once() -> Result<(), HookError> {
    if !HOOKED.load(Ordering::SeqCst) {
        create_hook()?;
        HOOKED.store(true, Ordering::SeqCst)
    }
    Ok(())
}

fn create_hook() -> Result<(), HookError> {
    unsafe {
        init_mh();

        let func_addr = AddressRepository::get_instance()
            .lock()
            .unwrap()
            .get_address(address::chat::MessageSent)
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

pub struct InputDispatchHook {
    inner_id: u64,
    position: Option<CallbackPosition>,
}

impl HookHandle for InputDispatchHook {
    type Args = Args;

    fn set_hook<F>(&mut self, position: CallbackPosition, f: F) -> Result<(), HookError>
    where
        F: Fn(Self::Args) + 'static + Send + Sync,
    {
        if position != CallbackPosition::Before {
            return Err(HookError::UnsupportedPosition);
        }

        hook_once()?;
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
}

impl Default for InputDispatchHook {
    fn default() -> Self {
        Self::new()
    }
}

impl InputDispatchHook {
    pub fn new() -> InputDispatchHook {
        InputDispatchHook {
            inner_id: rand::thread_rng().next_u64(),
            position: None,
        }
    }
}
