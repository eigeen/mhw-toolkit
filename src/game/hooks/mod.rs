mod action;
mod chat;
mod hit;
mod monster;

use std::sync::Once;

pub use action::*;
pub use chat::*;
pub use hit::*;
pub use monster::*;

use thiserror::Error;

static INITIALIZE_ONCE: Once = Once::new();

#[derive(Error, Debug)]
pub enum HookError {
    #[error("failed to create hook (code {0})")]
    CreateHook(i32),
    #[error("hook not set")]
    HookNotSet,
    #[error("the hook position is unsuppported")]
    UnsupportedPosition,
    #[error("cannot find address of {0}")]
    CannotFindAddress(String),
}

/// 初始化 MinHook 库
///
/// 初始化必须使用该函数。该函数确保进程整个生命周期内最多只会初始化一次。
pub fn init_mh() {
    INITIALIZE_ONCE.call_once(|| unsafe {
        minhook_sys::MH_Initialize();
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CallbackPosition {
    Before,
    After,
}

pub trait HookHandle {
    type Args;

    fn set_hook<F>(&mut self, position: CallbackPosition, f: F) -> Result<(), HookError>
    where
        F: Fn(Self::Args) + 'static + Send + Sync;

    fn unset_hook(&mut self) -> Result<(), HookError>;

    fn is_hooked(&self) -> bool;

    fn skip_call(&self, skip: bool) -> bool {
        skip
    } // with default implementation
}
