use std::{
    collections::HashMap,
    ffi::{c_void, OsString},
    os::windows::ffi::OsStringExt,
    ptr::{self, addr_of_mut},
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
};

use once_cell::sync::Lazy;
use rand::RngCore;
use winapi::ctypes::wchar_t;

use super::{init_mh, CallbackPosition, HookError, HookHandle};

type InputDispatchFunction = extern "C" fn(*const wchar_t);
type Args = String;
type CallbackFn = Box<dyn Fn(Args) + 'static + Send + Sync>;
type CallbacksTable = HashMap<CallbackPosition, Vec<(u64, CallbackFn)>>;

static mut ORIGINAL_FUNCTION: *mut c_void = ptr::null_mut();
static HOOKED: AtomicBool = AtomicBool::new(false);
static HOOK_CALLBACKS: Lazy<Mutex<CallbacksTable>> = Lazy::new(|| Mutex::new(HashMap::new()));

extern "C" fn hooked_function(inputs_ptr: *const wchar_t) {
    let inputs = unsafe {
        let raw_slice = std::slice::from_raw_parts(inputs_ptr, 128);
        let wchar_slice = raw_slice
            .iter()
            .position(|&c| c == 0)
            .map_or_else(|| raw_slice, |idx| &raw_slice[..idx]);
        OsString::from_wide(wchar_slice)
            .into_string()
            .unwrap_or_else(|_| "".to_string())
    };
    // Before
    if let Some(callbacks) = HOOK_CALLBACKS
        .lock()
        .unwrap()
        .get(&CallbackPosition::Before)
    {
        callbacks.iter().for_each(|(_, f)| f(inputs.clone()))
    }
    // 调用原始函数
    unsafe {
        let original: InputDispatchFunction = std::mem::transmute(ORIGINAL_FUNCTION);
        original(inputs_ptr);
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

        let target_function: *mut c_void = 0x14239D640 as *mut c_void;

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
