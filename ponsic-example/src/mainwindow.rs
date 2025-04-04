use lazy_static::lazy_static;
use ponsic::*;

use crate::button::BUTTON_DOWN;

fn main_window_process(Events { event, .. }: Events) -> Option<isize> {
    match event {
        Event::Destroy => {
            println!("MainWindow destroyed");
            App::should_exit(0);
            Some(0)
        }
        Event::UserDef {
            msg: BUTTON_DOWN, ..
        } => {
            println!("MainWindow received message ButtonDown");
            Some(0)
        }
        _ => None,
    }
}

lazy_static! {
    pub static ref MAINWINDOW_CLASS: Class = Registrar::new("MainWindow")
        .set_cursor(Cursor::Arrow)
        .set_process(wndproc!(();main_window_process))
        .build()
        .unwrap();
}