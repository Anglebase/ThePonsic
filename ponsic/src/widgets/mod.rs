mod main_window;
pub use main_window::MainWindow;

/// 此宏用于创建内部名称
#[macro_export]
macro_rules! ponsic_name {
    ($name:expr) => {
        concat!("__Ponsic_DEF_", $name)
    };
}