#[cfg(not(target_os = "windows"))]
fn main() {}

#[cfg(target_os = "windows")]
use ponsic_winsafe::*;

#[cfg(target_os = "windows")]
fn main() -> Result<(), SystemError> {
    let class = Registrar::new("HelloWorld")
        .set_cursor(Cursor::Arrow)
        .set_process(wndproc!(();|Events { event,.. }|{
            if let Event::Mouse { button: Button::Left, status: ButtonStatus::Down, .. } = event {
                println!("Hello Ponsic!");
                return Return::Finish
            }
            if let Event::Window(WindowEvent::Destroy) = event {
                App::should_exit(0);
                return Return::Finish
            }
            Return::Default
        }))
        .build()?;

    let window = class
        .make_window(Rect::from((Point::new(100, 100), Size::new(800, 600))))
        .set_style(&[WindowStyle::OverlappedWindow])
        .set_title("Hello Ponsic!")
        .build()?;

    window.show();

    while App::handle_event(true).unwrap() {}

    Ok(())
}
