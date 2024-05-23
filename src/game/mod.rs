pub mod mt_types;
pub mod resources;

#[cfg(feature = "hooks")]
pub mod hooks;

pub mod prelude {
    pub use crate::game::mt_types::*;
}
