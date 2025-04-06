mod main_window;
mod push_button;

pub use main_window::MainWindow;
pub use push_button::PushButton;

/// 此宏用于创建内部名称
#[macro_export]
macro_rules! ponsic_name {
    ($name:expr) => {
        concat!("__Ponsic_DEF_", $name)
    };
}

#[macro_export]
macro_rules! get_class {
    ($e:expr) => {
        $e.as_ref().map(|ok| ok.clone()).map_err(|err| err.clone())
    };
}
