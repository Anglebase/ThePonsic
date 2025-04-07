#[allow(unused)]
use std::thread::spawn;

#[cfg(target_os = "windows")]
use ponsic_winsafe::{graphics::context_2d::Rect, wndproc, *};

#[cfg(target_os = "windows")]
fn process(Events { event, window }: Events) -> Return {
    match event {
        Event::Window(WindowEvent::Destroy) => {
            App::should_exit(0);
            Return::Finish
        }
        Event::Other { .. } => Return::Default,
        e @ _ => {
            println!("{}", window.title());
            println!("{:?}", e);
            Return::Default
        }
    }
}

#[cfg(target_os = "windows")]
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
    let id = window.id();

    let join = spawn(move || {
        let window = class
            .make_window(Rect::from_ps(100, 100, 400, 300))
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
#[cfg(not(target_os = "windows"))]
fn main(){}