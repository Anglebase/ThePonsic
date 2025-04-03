use lazy_static::lazy_static;
use ponsic::*;

fn main_window_process(Events { event, .. }: Events) -> Option<isize> {
    match event {
        Event::Destroy => {
            App::should_exit(0);
            Some(0)
        }
        _ => None,
    }
}

lazy_static! {
    pub static ref MAINWINDOW_BUIDER: Class = Registrar::new("MainWindow")
        .set_cursor(Cursor::Arrow)
        .set_process(wndproc!(main_window_process))
        .build()
        .unwrap();
}
