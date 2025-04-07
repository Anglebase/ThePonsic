#[cfg(target_os = "windows")]
use ponsic_winsafe::{graphics::context_2d::Rect, wndproc, *};

#[cfg(target_os = "windows")]
fn process(Events { event, .. }: Events) -> Return {
    match event {
        Event::Other { msg, .. } => {
            let s = translate_msg(msg);
            println!("{}", s);
            Return::Default
        }
        e @ _ => {
            println!("{:?}", e);
            if let Event::Window(WindowEvent::Destroy) = e {
                App::should_exit(0);
                Return::Finish
            } else {
                Return::Default
            }
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
    while App::handle_event(true).unwrap_or(true) {}
}

#[cfg(not(target_os = "windows"))]
fn main(){}
