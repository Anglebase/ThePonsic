#[cfg(not(target_os = "windows"))]
fn main() {}

#[cfg(target_os = "windows")]
use ponsic_winsafe::*;

#[cfg(target_os = "windows")]
struct My {
    timer: Option<Timer>,
}
#[cfg(target_os = "windows")]
fn main() -> Result<(), SystemError> {
    use std::time::Duration;

    let class = Registrar::new("TimerExp")
        .set_cursor(Cursor::Arrow)
        .set_process(wndproc!(My;|Events { event,window },mut data: The<My>|{
            match event {
                Event::Mouse { button, status: ButtonStatus::Down, .. } => {
                    match button {
                        Button::Left => {
                            let timer = window.set_timer(Duration::from_secs(1)).unwrap();
                            if let Some(mut my) = data.as_mut() {
                                my.timer = Some(timer);
                            }
                        },
                        Button::Right => {
                            if let Some(mut my) = data.as_mut() {
                                if let Some(timer) = my.timer.take() {
                                    timer.kill().unwrap();
                                }
                            }
                        },
                        _ => {}
                    }
                    Return::Finish
                }
                Event::Timer { id } => {
                    println!("Timer {:?} expired", id);
                    Return::Finish
                }
                Event::Window(WindowEvent::Destroy) => {
                    App::should_exit(0);
                    Return::Finish
                }
                _ => Return::Default
            }
        }))
        .build()?;

    let window = class
        .make_window(Rect::from((Point::new(100, 100), Size::new(800, 600))))
        .set_style(&[WindowStyle::OverlappedWindow])
        .set_title("Hello Ponsic!")
        .bind_data(My { timer: None })
        .build()?;

    window.show();

    while App::handle_event(true).unwrap() {}

    Ok(())
}
