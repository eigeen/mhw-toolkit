mod action;
mod monster;

use std::sync::Once;

pub use action::*;
pub use monster::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HookError {
    #[error("failed to create hook (code {0})")]
    CreateHook(i32),
}

static INITIALIZE_ONCE: Once = Once::new();

pub fn init_mh() {
    INITIALIZE_ONCE.call_once(|| unsafe {
        minhook_sys::MH_Initialize();
    });
}
