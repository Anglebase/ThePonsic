use std::thread::spawn;

use ponsic_winsafe::{graphics::context_2d::Rect, wndproc, *};

fn process(Events { event, window }: Events) -> Option<isize> {
    match event {
        Event::Destroy => {
            App::should_exit(0);
            Some(0)
        }
        Event::Other { .. } => None,
        e @ _ => {
            println!("{}", window.title());
            println!("{:?}", e);
            None
        }
    }
}

fn main() {
    let class = Registrar::new("MyApp")
        .set_cursor(Cursor::Arrow)
        .set_process(wndproc!(process))
        .build()
        .unwrap();

    let window = class
        .window_builder(Rect::from_ps(100, 100, 800, 600))
        .set_title("MyApp")
        .set_style(&[WindowStyle::OverlappedWindow])
        .build()
        .unwrap();

    window.show();
    let id = window.id();

    let join = spawn(move || {
        let window = class
            .window_builder(Rect::from_ps(100, 100, 400, 300))
            .set_title("MyApp2")
            .set_parent(id)
            .set_style(&[WindowStyle::OverlappedWindow])
            .build()
            .unwrap();

        window.show();

        while App::handle_event(true).unwrap_or(true) {}
    });

    while App::handle_event(true).unwrap_or(true) {}

    join.join().unwrap();
}
