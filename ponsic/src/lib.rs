mod utils;

/// 预定义窗口结构体
#[cfg(target_os = "windows")]
pub mod widgets;

#[cfg(target_os = "windows")]
pub use ponsic_winsafe::*;

