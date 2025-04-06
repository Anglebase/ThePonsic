mod utils;

#[cfg(target_os = "windows")]
pub mod widgets;

#[cfg(target_os = "windows")]
pub use ponsic_winsafe::*;

pub use inherits::inherits;
pub use lazy_static::lazy_static;
