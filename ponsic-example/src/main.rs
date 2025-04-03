use ponsic::{App, WindowManager, WindowStyle};
use ponsic_example::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let window = MAINWINDOW_BUIDER
        .window_builder(10, 10, 800, 600)
        .set_title("Ponsic 应用程序")
        .set_style(&[WindowStyle::OverlappedWindow])
        .build()?;

    window.show();
    while App::handle_event(true).unwrap_or(true) {}
    Ok(())
}
