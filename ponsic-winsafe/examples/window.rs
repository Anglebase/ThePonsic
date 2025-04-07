use ponsic_winsafe::{graphics::context_2d::Rect, wndproc, *};

fn process(Events { event, .. }: Events) -> Return {
    match event {
        Event::Create => {
            // println!("Created");
            Return::Finish
        }
        Event::Destroy => {
            App::should_exit(0);
            Return::Finish
        }
        Event::Other { msg, .. } => {
            let s = translate_msg(msg);
            if s == "WM_NCMOUSEMOVE" || s == "WM_NCMOUSELEAVE" {
                println!("{}", s);
            }
            Return::Default
        }
        Event::SizeRange {
            min_track_width,
            min_track_height,
            ..
        } => {
            *min_track_width = 640;
            *min_track_height = 480;
            Return::Finish
        }
        Event::SizeChanging {
            type_: SizingSide::Unknown(a),
            ..
        } => {
            println!("{}", a);
            panic!("Unknown SizingSide");
        }
        e @ _ => {
            println!("{:?}", e);
            Return::Default
        }
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
