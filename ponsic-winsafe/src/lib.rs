#[cfg(target_os = "windows")]
mod events;
#[cfg(target_os = "windows")]
mod safe_proc;
#[cfg(target_os = "windows")]
mod win;
#[cfg(target_os = "windows")]
mod the;
#[cfg(target_os = "windows")]
pub mod graphics;

#[cfg(target_os = "windows")]
pub use events::*;
#[cfg(target_os = "windows")]
pub use safe_proc::*;
#[cfg(target_os = "windows")]
pub use win::{
    app::App,
    class::*,
    window::*,
    error::*,
    gen_by_py::translate_msg,
};
#[cfg(target_os = "windows")]
pub use the::*;