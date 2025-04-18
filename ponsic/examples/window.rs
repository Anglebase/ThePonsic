#[cfg(target_os = "windows")]
use ponsic::{App, Event, Events, Return, WindowEvent, widgets::Proc};
use ponsic::{
    Recti,
    graphics::context_2d::{Context2D, DrawText},
};

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

    fn draw(&mut self, context: ponsic::graphics::Context) {
        let context: Context2D = context.into();
        context.draw_text("Hello World", &mut Recti::new(10, 10, 100, 100), &[]);
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
