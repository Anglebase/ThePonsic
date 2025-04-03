use std::error::Error;

use ponsic::*;

fn process(Events { event, .. }: Events) -> Option<isize> {
    match event {
        Event::Destroy => {
            App::should_exit(0);
            Some(0)
        }
        Event::Mouse { button, status, .. } => {
            if let (Button::Left, ButtonStatus::Down) = (button, status) {
                println!("Left mouse button pressed");
            }
            Some(0)
        }
        _ => None,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let class = Registrar::new("AppMain")
        .set_cursor(Cursor::Arrow)
        .set_process(wndproc!(process))
        .build()?;

    let window = class
        .window_builder(10, 10, 800, 600)
        .set_title("MainWindow")
        .set_style(&[WindowStyle::OverlappedWindow])
        .build()?;

    window.show();

    while App::handle_event(true).unwrap_or(true) {}
    Ok(())
}
