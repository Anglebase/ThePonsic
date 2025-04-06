use gom::Registry;
use inherits::inherits;
use lazy_static::lazy_static;
use ponsic::{
    graphics::context_2d::{Context2D, DrawClose, Rect},
    *,
};
fn paint(context: Context2D<'_>) {
    context.clear();
    context.rectangle(Rect {
        left: 0,
        top: 0,
        right: 100,
        bottom: 50,
    });
}
pub const BUTTON_DOWN: u32 = 0;
fn button_process(Events { event, .. }: Events) -> Option<isize> {
    match event {
        Event::Destroy => {
            println!("Button destroyed");
            None
        }
        Event::Paint { context } => {
            paint(context.into());
            Some(0)
        }
        Event::Mouse { button, status, .. }
            if (ponsic::Button::Left, ButtonStatus::Down) == (button, status) =>
        {
            println!("Button pressed");
            Registry::with("MainWindow", |id: &WindowId| {
                unsafe { Window::post(*id, BUTTON_DOWN, 0, 0).unwrap() };
            });
            Some(0)
        }
        _ => None,
    }
}

lazy_static! {
    pub static ref BUTTON_CLASS: Class = Registrar::new("Button")
        .set_cursor(Cursor::Arrow)
        .set_process(wndproc!(();button_process))
        .build()
        .unwrap();
}

#[inherits(Window)]
pub struct Button {}

impl Button {
    pub fn new(rect: Rect, parent: WindowId) -> Result<Self, SystemError> {
        let button = BUTTON_CLASS
            .window_builder(rect)
            .set_parent(parent)
            .set_style(&[WindowStyle::Child])
            .set_title("Button")
            .build()?;
        Ok(Self { parent: button })
    }
}
