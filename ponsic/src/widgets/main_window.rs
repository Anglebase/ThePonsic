use inherits::inherits;
use lazy_static::lazy_static;
use ponsic_winsafe::{
    cast, graphics::context_2d::Rect, wndproc, App, Class, Event, Events, Registrar, Return, SystemError, The, Window, WindowManager, WindowStyle
};

use crate::{get_class, ponsic_name};

fn main_window_process(Events { event, .. }: Events, _data: The<MainWindowData>) -> Return {
    match event {
        Event::Destroy => {
            App::should_exit(0);
            Return::Finish
        }
        _ => Return::Default,
    }
}

lazy_static! {
    static ref MAINWINDOW_CLASS: Result<Class, SystemError> =
        Registrar::new(ponsic_name!("MainWindow"))
            .set_cursor(ponsic_winsafe::Cursor::Arrow)
            .set_process(wndproc!(MainWindowData;main_window_process))
            .build();
}

#[inherits(Window)]
pub struct MainWindow {
    #[allow(unused)]
    data: The<MainWindowData>,
}
pub struct MainWindowData {}

impl MainWindow {
    pub fn new(rect: Rect, title: &str) -> Result<Self, SystemError> {
        let class = get_class!(MAINWINDOW_CLASS)?;

        let data = MainWindowData {};

        let window = class
            .window_builder(rect)
            .set_title(title)
            .set_style(&[WindowStyle::OverlappedWindow])
            .bind(data)
            .build()?;

        let data = unsafe { cast::<MainWindowData>(window.id()) };

        Ok(Self {
            data,
            parent: window,
        })
    }
}
