use gom::Registry;
use ponsic::{App, WindowManager, WindowStyle};
use ponsic_example::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let window = MAINWINDOW_CLASS
        .window_builder(10, 10, 800, 600)
        .set_title("Ponsic 应用程序")
        .set_style(&[WindowStyle::OverlappedWindow])
        .build()?;

    Registry::register("MainWindow", window.id()).unwrap();

    let button = BUTTON_CLASS
        .window_builder(10, 10, 100, 50)
        .set_parent(window.id())
        .set_style(&[WindowStyle::Child])
        .set_title("Button")
        .build()?;

    window.show();
    button.show();
    while App::handle_event(true).unwrap_or(true) {}
    Ok(())
}
