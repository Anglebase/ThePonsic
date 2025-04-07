use ponsic_winsafe::{graphics::context_2d::*, wndproc, *};

fn paint(context: FastContext2D) {
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

    context.rectangle(Rect {
        left: 200,
        top: 200,
        right: 250,
        bottom: 300,
    });

    let mut rect = Rect::from_pos_size(
        Point { x: 10, y: 10 },
        Size {
            width: 100,
            height: 50,
        },
    );

    context.draw_text(
        "123",
        &mut rect,
        &[
            DrawTextMode::Center,
            DrawTextMode::VCenter,
            DrawTextMode::SingleLine,
        ],
    );
}

fn process(Events { event, .. }: Events) -> Return {
    match event {
        Event::Window(WindowEvent::Destroy) => {
            App::should_exit(0);
            Return::Finish
        }
        Event::Paint { context } => {
            paint(context.into());
            Return::Finish
        }
        _ => Return::Default,
    }
}

fn main() {
    let class = Registrar::new("MyApp")
        .set_cursor(Cursor::Arrow)
        .set_process(wndproc!(();process))
        .build()
        .unwrap();

    let window = class
        .make_window(Rect::from_ps(100, 100, 800, 600))
        .set_title("MyApp")
        .set_style(&[WindowStyle::OverlappedWindow])
        .build()
        .unwrap();

    window.show();
    while App::handle_event(true).unwrap_or(true) {}
}
