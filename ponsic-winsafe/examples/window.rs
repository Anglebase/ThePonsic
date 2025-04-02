use ponsic_winsafe::{wndproc, *};

fn process(Events { event, .. }: Events) -> Option<isize> {
    match event {
        Event::Destroy => {
            App::should_exit(0);
            Some(0)
        }
        Event::Other { .. } => None,
        e @ _ => {
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
        .window_builder(100, 100, 800, 600)
        .set_title("MyApp")
        .set_style(&[WindowStyle::OverlappedWindow])
        .build()
        .unwrap();

    window.show();
    App::handle_events();
}
