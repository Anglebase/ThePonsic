#[cfg(not(target_os = "windows"))]
fn main() {}

use ponsic_winsafe::graphics::context_2d::Context2D;
#[cfg(target_os = "windows")]
use ponsic_winsafe::*;

#[cfg(target_os = "windows")]
fn paint(mut context: Context2D) {
    use ponsic_winsafe::graphics::context_2d::{DrawOpen, LineStyle};

    context.clear();
    context.set_line_width(10);

    let style = [
        LineStyle::Solid,
        LineStyle::Dash,
        LineStyle::DashDot,
        LineStyle::Dot,
        LineStyle::DashDotDot,
    ];

    for (i, &style) in style.iter().enumerate() {
        let i = i as i32;
        context.set_line_style(style);
        context.line(Point::new(10, 100 + i * 100), Point::new(790, 100 + i * 100));
    }
}

#[cfg(target_os = "windows")]
fn my_proc(Events { event, .. }: Events) -> Return {
    if let Event::Paint { context } = event {
        paint(context.into());
        return Return::Finish
    }
    if let Event::Window(WindowEvent::Destroy) = event {
        App::should_exit(0);
        return Return::Finish
    }
    Return::Default
}

#[cfg(target_os = "windows")]
fn main() -> Result<(), SystemError> {
    let class = Registrar::new("PaintContext")
        .set_cursor(Cursor::Arrow)
        .set_process(wndproc!(();my_proc))
        .build()?;

    let window = class
        .make_window(Rect::from((Point::new(100, 100), Size::new(800, 600))))
        .set_style(&[WindowStyle::OverlappedWindow])
        .set_title("Paint Context Example")
        .build()?;

    window.show();

    while App::handle_event(true).unwrap() {}

    Ok(())
}
