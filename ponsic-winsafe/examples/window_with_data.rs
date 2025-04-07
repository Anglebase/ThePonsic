#[cfg(target_os = "windows")]
use ponsic_winsafe::{graphics::context_2d::Rect, wndproc, *};

#[cfg(target_os = "windows")]
#[derive(Debug)]
struct Data {
    a: u32,
    b: String,
}

#[cfg(target_os = "windows")]
fn process(Events { event, .. }: Events, mut the: The<Data>) -> Return {
    match event {
        Event::Window(WindowEvent::Create) => {
            println!("Created");
            Return::Finish
        }
        Event::Mouse { button, status, .. } => {
            if (button, status) == (Button::Left, ButtonStatus::Down) {
                println!("Clicked!");
                if let Some(mut r) = the.as_mut() {
                    r.a += 1;
                    r.b += " World";
                    println!("The value is now {:?}", *r);
                }
            }
            Return::Finish
        }
        Event::Window(WindowEvent::Destroy) => {
            App::should_exit(0);
            Return::Finish
        }
        _ => Return::Default,
    }
}

#[cfg(target_os = "windows")]
fn main() {
    let class = Registrar::new("MyApp")
        .set_cursor(Cursor::Arrow)
        .set_process(wndproc!(Data;process))
        .build()
        .unwrap();

    let window = class
        .make_window(Rect::from_ps(100, 100, 800, 600))
        .set_title("MyApp")
        .set_style(&[WindowStyle::OverlappedWindow])
        .bind_data(Data {
            a: 10,
            b: "Hello".into(),
        })
        .build()
        .unwrap();
    window.show();
    while App::handle_event(true).unwrap_or(true) {}
}

#[cfg(not(target_os = "windows"))]
fn main(){}