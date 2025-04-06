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
fn button_process(Events { event, .. }: Events, mut the: The<Call>) -> Return {
    match event {
        Event::Destroy => {
            println!("Button destroyed");
            Return::Default
        }
        Event::Paint { context } => {
            paint(context.into());
            Return::Finish
        }
        Event::Mouse { button, status, .. }
            if (ponsic::Button::Left, ButtonStatus::Down) == (button, status) =>
        {
            if let Some(mut the) = the.as_mut() {
                let f = the.callback.as_mut();
                (*f)(true);
            }
            Return::Finish
        }
        Event::Mouse { button, status, .. }
            if (ponsic::Button::Left, ButtonStatus::Up) == (button, status) =>
        {
            if let Some(mut the) = the.as_mut() {
                let f = the.callback.as_mut();
                (*f)(false);
            }
            Return::Finish
        }
        _ => Return::Default,
    }
}

lazy_static! {
    pub static ref BUTTON_CLASS: Class = Registrar::new("Button")
        .set_cursor(Cursor::Arrow)
        .set_process(wndproc!(Call;button_process))
        .build()
        .unwrap();
}

#[inherits(Window)]
pub struct Button {
    the: The<Call>,
}

impl Button {
    pub fn new(rect: Rect, parent: WindowId) -> Result<Self, SystemError> {
        let c = Call {
            callback: Box::new(|_| {}),
        };

        let button = BUTTON_CLASS
            .window_builder(rect)
            .set_parent(parent)
            .set_style(&[WindowStyle::Child])
            .set_title("Button")
            .bind(c)
            .build()?;

        let the = assert_cast::<Call>(button.id());

        Ok(Self {
            parent: button,
            the,
        })
    }

    pub fn set_callback(&mut self, callback: impl FnMut(bool) + 'static) {
        if let Some(mut the) = self.the.as_mut() {
            the.callback = Box::new(callback);
        }
    }
}

pub struct Call {
    callback: Box<dyn FnMut(bool) + 'static>,
}
