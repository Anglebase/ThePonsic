#[cfg(target_os = "windows")]
use ponsic::{App, Event, Events, Return, WindowEvent, widgets::Proc};

#[cfg(not(target_os = "windows"))]
fn main() {}

struct MyWindow {}
impl Proc for MyWindow {
    fn handle(&mut self, Events { event, .. }: Events) -> Return {
        if let Event::Window(WindowEvent::Destroy) = event {
            App::should_exit(0);
            return Return::Finish;
        }
        Return::Default
    }
}

#[cfg(target_os = "windows")]
fn main() {
    use ponsic::{App, Point, Rect, Size, WindowManager, widgets::Window};

    let window = Window::create(
        Rect::from((Point::new(100, 100), Size::new(800, 600))),
        "Hello World",
        None,
        MyWindow {},
    )
    .unwrap();

    window.show();

    while App::handle_event(true).unwrap() {}
}
