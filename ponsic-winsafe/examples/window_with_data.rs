use ponsic_winsafe::{graphics::context_2d::Rect, wndproc, *};

fn process(Events { event, .. }: Events, mut the: The<u32>) -> Option<isize> {
    match event {
        Event::Create => {
            println!("Created");
            Some(0)
        }
        Event::Mouse { button, status, .. } => {
            if (button, status) == (Button::Left, ButtonStatus::Down) {
                println!("Clicked!");
                if let Some(mut r) = the.as_mut() {
                    *r += 1;
                    println!("The value is now {}", *r);
                }
            }
            Some(0)
        }
        Event::Destroy => {
            App::should_exit(0);
            Some(0)
        }
        _ => None,
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
        .bind(10u32)
        .build()
        .unwrap();
    window.show();
    while App::handle_event(true).unwrap_or(true) {}
}
