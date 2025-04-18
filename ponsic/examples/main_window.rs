use ponsic::{widgets::MainWindow, App, Point, Rect, Size, SystemError, WindowManager};

fn main() -> Result<(), SystemError> {
    let window = MainWindow::new(
        Rect::from((Point::new(100, 100), Size::new(800, 600))),
        "MyApplication",
    )?;
    window.show();

    while App::handle_event(true).unwrap() {}
    Ok(())
}
