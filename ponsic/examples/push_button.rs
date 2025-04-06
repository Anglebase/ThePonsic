use ponsic::{
    App, SystemError, WindowManager,
    graphics::context_2d::Rect,
    widgets::{MainWindow, PushButton},
};

fn main() -> Result<(), SystemError> {
    let main_window = MainWindow::new(Rect::from_ps(100, 100, 800, 600), "New Ponsic Window")?;
    let mut button = PushButton::new(Rect::from_ps(50, 50, 100, 50), "PushButton", &main_window)?;
    button.add_callback(|b| {
        println!("PushButton pressed!: {}", b);
    });
    main_window.show();
    button.show();
    while App::handle_event(true).unwrap_or(true) {}
    Ok(())
}
