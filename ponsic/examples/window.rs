use ponsic::{widgets::Item, App};

#[cfg(not(target_os = "windows"))]
fn main() {}

struct MainWindow {}
impl Item for MainWindow {
    fn destroy(&mut self, handle: ponsic::WindowHandle) {
        println!("MainWindow destroyed: {:?}", handle);
        App::should_exit(0);
    }
}

#[cfg(target_os = "windows")]
fn main() {
    use ponsic::{App, Point, Rect, Size, WindowManager, widgets::Window};

    let window = Window::new(
        Rect::from((Point::new(100, 100), Size::new(800, 600))),
        "Hello World",
        None,
        MainWindow {},
    )
    .unwrap();

    window.show();

    while App::handle_event(true).unwrap() {}
}
