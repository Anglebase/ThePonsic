use ponsic_winsafe::{graphics::context_2d::Rect, wndproc, *};

fn process(Events { event, .. }: Events) -> Option<isize> {
    match event {
        Event::Create => {
            // println!("Created");
            Some(0)
        }
        Event::Destroy => {
            App::should_exit(0);
            Some(0)
        }
        Event::Other { msg, .. } => {
            let s = translate_msg(msg);
            if s == "UNDEFINED" {
                println!("{}", msg);
            }
            None
        }
        Event::SizeRange {
            min_track_width,
            min_track_height,
            ..
        } => {
            *min_track_width = 640;
            *min_track_height = 480;
            Some(0)
        }
        _ => {
            // println!("{:?}", e);
            None
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
        .window_builder(Rect::from_ps(100, 100, 800, 600))
        .set_title("MyApp")
        .set_style(&[WindowStyle::OverlappedWindow])
        .build()
        .unwrap();

    window.show();
    while App::handle_event(true).unwrap_or(true) {}
}
