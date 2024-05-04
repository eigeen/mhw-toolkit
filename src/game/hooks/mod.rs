mod action;
mod monster;

pub use action::*;
pub use monster::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HookError {
    #[error("failed to create hook (code {0})")]
    CreateHook(i32),
}
