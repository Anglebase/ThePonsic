use ponsic_winsafe::{graphics::context_2d::Rect, wndproc, *};

fn process(Events { event, .. }: Events, the: The<u32>) -> Option<isize> {
    match the.as_ref() {
        Some(r) => {
            println!("{}", *r);
        }
        _ => {}
    }
    match event {
        Event::Create => {
            println!("Created");
            Some(0)
        }
        Event::Destroy => {
            App::should_exit(0);
            Some(0)
        }
        Event::Other { msg, .. } => {
            println!("{}", translate_msg(msg));
            None
        }
        e @ _ => {
            println!("{:?}", e);
            None
        }
    }
}

fn main() {
    let class = Registrar::new("MyApp")
        .set_cursor(Cursor::Arrow)
        .set_process(wndproc!(u32;process))
        .build()
        .unwrap();

    let window = class
        .window_builder(Rect::from_ps(100, 100, 800, 600))
        .set_title("MyApp")
        .set_style(&[WindowStyle::OverlappedWindow])
        .build()
        .unwrap();
    window.bind(10u32);
    window.show();
    while App::handle_event(true).unwrap_or(true) {}
}
