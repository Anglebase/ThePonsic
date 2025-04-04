use ponsic_winsafe::{graphics::context_2d::*, wndproc, *};

fn paint(mut context: Context2D<'_>) {
    context.clear();
    context.set_line_width(10);
    context.set_line_color(Color::from_rgb(255, 0, 0));
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
    context.set_join_style(JoinStyle::Round);
    context.arc(rect, p1, p2);
    context.polyline(&rect.to_polyline());

    context.set_line_style(LineStyle::Null);

    context.set_brush_color(Color::from_gray(128));
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

    context.set_font_height(16);
    context.set_font_name("宋体");

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
        .set_process(wndproc!(();process))
        .build()
        .unwrap();

    let window = class
        .window_builder(Rect::from_ps(100, 100, 800, 600))
        .set_title("MyApp")
        .set_style(&[WindowStyle::OverlappedWindow])
        .build()
        .unwrap();

    window.show();
    while App::handle_event(true).unwrap_or(true) {}
}
