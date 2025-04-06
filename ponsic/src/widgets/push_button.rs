use inherits::inherits;
use lazy_static::lazy_static;
use ponsic_winsafe::{
    Button,
    graphics::context_2d::{
        Brush, BrushSetter, Color, DrawClose, DrawText, DrawTextMode, EnvironmentSetter,
        FastContext2D, GenBrush, Point, Rect,
    },
    *,
};

use crate::get_class;

lazy_static! {
    static ref BUTTON_CLASS: Result<Class, SystemError> = Registrar::new("Button")
        .set_cursor(Cursor::Hand)
        .set_process(wndproc!(PushButtonData;button_process))
        .build();
}

fn paint(mut context: FastContext2D, data: &The<PushButtonData>) {
    context.set_back_mode(graphics::context_2d::BackMode::Transparent);
    if let Some(data) = data.as_ref() {
        let mut rect = Rect::from_pos_size(Point::from_xy(0, 0), data.rect.size());
        if data.state {
            context.set_brush(&data.brush_true);
        } else {
            context.set_brush(&data.brush_false);
        }
        context.rectangle(rect);
        context.draw_text(
            &data.text,
            &mut rect,
            &[
                DrawTextMode::Center,
                DrawTextMode::VCenter,
                DrawTextMode::SingleLine,
            ],
        );
    }
}

fn button_process(Events { event, window }: Events, mut data: The<PushButtonData>) -> Return {
    match event {
        Event::Paint { context } => {
            paint(context.into(), &data);
            Return::Finish
        }
        Event::Mouse {
            button: Button::Left,
            status,
            ..
        } => {
            if let Some(mut data) = data.as_mut() {
                if status == ButtonStatus::Down {
                    data.state = true;
                    data.callback.iter_mut().for_each(|callback| callback(true));
                    window.redraw();
                } else if status == ButtonStatus::Up {
                    data.state = false;
                    data.callback.iter_mut().for_each(|callback| callback(false));
                    window.redraw();
                }
            }
            Return::Finish
        }
        _ => Return::Default,
    }
}

#[inherits(Window)]
pub struct PushButton {
    data: The<PushButtonData>,
}

struct PushButtonData {
    state: bool,
    text: String,
    rect: Rect,
    brush_false: Brush,
    brush_true: Brush,
    callback: Vec<Box<dyn FnMut(bool) + 'static>>,
}

impl PushButton {
    pub fn new(rect: Rect, text: &str, parent: &Window) -> Result<Self, SystemError> {
        let class = get_class!(BUTTON_CLASS)?;
        let data = PushButtonData {
            state: false,
            text: text.to_string(),
            rect,
            brush_false: GenBrush::Solid(Color::from_gray(255)).create(),
            brush_true: GenBrush::Solid(Color::from_gray(198)).create(),
            callback: Vec::new(),
        };
        let window = class
            .window_builder(rect)
            .set_title(text)
            .set_parent(parent.id())
            .set_style(&[WindowStyle::Child])
            .bind(data)
            .build()?;
        let data = unsafe { cast::<PushButtonData>(window.id()) };
        Ok(Self {
            data,
            parent: window,
        })
    }

    pub fn add_callback<F>(&mut self, callback: F)
    where
        F: FnMut(bool) + 'static,
    {
        if let Some(mut data) = self.data.as_mut() {
            data.callback.push(Box::new(callback));
        }
    }
}
