use ponsic_winsafe::{
    graphics::context_2d::{Context2D, Point, Rect},
    wndproc, *,
};

fn paint(context: Context2D) {
    context.clear();
    context.line(Point::from_xy(10, 10), Point::from_xy(200, 200));
    let points = [
        Point::from_xy(10, 5),
        Point::from_xy(200, 20),
        Point::from_xy(30, 300),
    ];
    context.polyline(&points);

    let rect = Rect {
        left: 400,
        top: 20,
        right: 450,
        bottom: 100,
    };
    let p1 = Point::from_xy(rect.left, rect.top);
    let p2 = Point::from_xy(rect.right, rect.bottom);
    context.arc(rect, p1, p2);
    context.polyline(&rect.to_polyline());
}

fn process(Events { event, .. }: Events) -> Option<isize> {
    match event {
        Event::Destroy => {
            App::should_exit(0);
            Some(0)
        }
        Event::Paint { context } => {
            paint(context.into());
            Some(0)
        }
        _ => None,
    }
}

fn main() {
    let class = Registrar::new("MyApp")
        .set_cursor(Cursor::Arrow)
        .set_process(wndproc!(process))
        .build()
        .unwrap();

    let window = class
        .window_builder(100, 100, 800, 600)
        .set_title("MyApp")
        .set_style(&[WindowStyle::OverlappedWindow])
        .build()
        .unwrap();

    window.show();
    while App::handle_event(true).unwrap_or(true) {}
}
