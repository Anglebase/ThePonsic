use inherits::inherits;
use lazy_static::lazy_static;
use ponsic::{graphics::context_2d::Rect, *};

use crate::button::BUTTON_DOWN;

fn main_window_process(Events { event, .. }: Events) -> Return {
    match event {
        Event::Destroy => {
            println!("MainWindow destroyed");
            App::should_exit(0);
            Return::Finish
        }
        Event::UserDef {
            msg: BUTTON_DOWN, ..
        } => {
            println!("MainWindow received message ButtonDown");
            Return::Finish
        }
        _ => Return::Default,
    }
}

lazy_static! {
    pub static ref MAINWINDOW_CLASS: Class = Registrar::new("MainWindow")
        .set_cursor(Cursor::Arrow)
        .set_process(wndproc!(();main_window_process))
        .build()
        .unwrap();
}

#[inherits(Window)]
pub struct MainWindow {}

impl MainWindow {
    pub fn new(rect: Rect, title: &str) -> Result<Self, SystemError> {
        let window = MAINWINDOW_CLASS
            .window_builder(rect)
            .set_title(title)
            .set_style(&[WindowStyle::OverlappedWindow])
            .build()?;
        Ok(Self { parent: window })
    }
}
