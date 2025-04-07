#[cfg(target_os = "windows")]
use ponsic::{App, SystemError, WindowManager, graphics::context_2d::Rect, widgets::MainWindow};

#[cfg(target_os = "windows")]
fn main() -> Result<(), SystemError> {
    let main_window = MainWindow::new(Rect::from_ps(100, 100, 800, 600), "New Ponsic Window")?;
    main_window.show();
    while App::handle_event(true).unwrap_or(true) {}
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn main(){}