#[cfg(target_os = "windows")]
mod events;
#[cfg(target_os = "windows")]
pub mod graphics;
#[cfg(target_os = "windows")]
mod safe_proc;
#[cfg(target_os = "windows")]
mod the;
#[cfg(target_os = "windows")]
mod win;
#[cfg(target_os = "windows")]
mod window_data;

#[cfg(target_os = "windows")]
pub use events::*;
#[cfg(target_os = "windows")]
pub use safe_proc::*;
#[cfg(target_os = "windows")]
pub use the::*;
#[cfg(target_os = "windows")]
pub use win::{app::App, class::*, error::*, gen_by_py::translate_msg, window::*};
#[cfg(target_os = "windows")]
pub use window_data::*;

pub use ponsic_color::*;
pub use ponsic_types::*;
