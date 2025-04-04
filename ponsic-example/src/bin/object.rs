use std::pin::Pin;

use ponsic::{
    graphics::context_2d::{Point, Rect, Size},
    *,
};
use ponsic_example::{BUTTON_CLASS, MAINWINDOW_CLASS};

#[inherits::inherits(Window)]
struct Button {}

impl Button {
    pub fn new(rect: Rect, parent: WindowId) -> Result<Self, SystemError> {
        let window = BUTTON_CLASS
            .window_builder(rect)
            .set_style(&[WindowStyle::Child])
            .set_parent(parent)
            .build()?;
        Ok(Self { parent: window })
    }
}

#[inherits::inherits(Window)]
struct MainWindow {}

impl MainWindow {
    pub fn new() -> Result<Pin<Self>, SystemError> {
        let window = MAINWINDOW_CLASS
            .window_builder(Rect::from_ps(100, 100, 800, 600))
            .set_style(&[WindowStyle::OverlappedWindow])
            .build()?;
        let ret = Pin::new(Self { parent: window });
        
        Ok(ret)
    }
}

fn main() {
    let main_window = MainWindow::new().unwrap();
    let button = Button::new(
        Rect::from_pos_size(Point::from_xy(100, 50), Size::from_wh(100, 50)),
        main_window.id(),
    )
    .unwrap();

    main_window.show();
    button.show();

    while App::handle_event(true).unwrap() {}
}