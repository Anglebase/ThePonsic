use ponsic::{App, WindowManager, graphics::context_2d::Rect};
use ponsic_example::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let window = MainWindow::new(Rect::from_ps(100, 100, 800, 600), "Ponsic 应用程序")?;

    let mut button = Button::new(Rect::from_ps(100, 100, 100, 50), window.id())?;
    button.set_callback(|s| {
        println!("Button pressed: {}", s);
    });

    window.show();
    button.show();
    while App::handle_event(true).unwrap_or(true) {}
    Ok(())
}
