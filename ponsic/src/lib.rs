mod utils;

#[cfg(target_os = "windows")]
pub mod widgets;

#[cfg(target_os = "windows")]
pub use ponsic_winsafe::*;

pub use lazy_static::lazy_static;
pub use inherits::inherits;