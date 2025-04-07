#[cfg(target_os = "windows")]
mod button;
#[cfg(target_os = "windows")]
mod mainwindow;

#[cfg(target_os = "windows")]
pub use button::Button;
#[cfg(target_os = "windows")]
pub use mainwindow::MainWindow;
