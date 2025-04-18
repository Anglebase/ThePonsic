use ponsic_winsafe::*;
use std::{
    any::type_name,
    ops::{Deref, DerefMut},
    sync::OnceLock,
};

static CLASS: OnceLock<Result<Class, SystemError>> = OnceLock::new();

fn window_proc(Events { event, window }: Events, mut data: The<WindowData>) -> Return {
    match event {
        Event::Window(WindowEvent::Close) => {
            if let Some(mut data) = data.as_mut() {
                data.item.destroy(window);
            }
            Return::Finish
        }
        _ => Return::Default,
    }
}

pub trait Item {
    fn destroy(&mut self, handle: WindowHandle) {
        println!("{:?} destroy", handle);
    }
}

pub struct WindowData {
    item: Box<dyn Item>,
}

pub struct Window {
    window: ponsic_winsafe::Window,
    data: The<WindowData>,
}

impl Deref for Window {
    type Target = ponsic_winsafe::Window;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl DerefMut for Window {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window
    }
}

impl Window {
    pub fn new<T: Item + 'static>(
        rect: Recti,
        title: &str,
        parent: Option<WindowId>,
        item: T,
    ) -> Result<Self, SystemError> {
        let class = CLASS
            .get_or_init(|| -> Result<Class, SystemError> {
                let class = Registrar::new(type_name::<Self>())
                    .set_process(wndproc!(WindowData;window_proc))
                    .set_cursor(ponsic_winsafe::Cursor::Arrow)
                    .build()?;
                Ok(class)
            })
            .clone()?;

        let data = WindowData {
            item: Box::new(item),
        };
        let window = if let Some(parent) = parent {
            class
                .make_window(rect)
                .set_title(title)
                .set_parent(parent)
                .set_style(&[WindowStyle::Child])
                .bind_data(data)
                .build()?
        } else {
            class
                .make_window(rect)
                .set_title(title)
                .set_style(&[WindowStyle::OverlappedWindow])
                .bind_data(data)
                .build()?
        };
        let data = window.data();
        Ok(Self { window, data })
    }
}

impl AsRef<The<WindowData>> for Window {
    fn as_ref(&self) -> &The<WindowData> {
        &self.data
    }
}

impl AsMut<The<WindowData>> for Window {
    fn as_mut(&mut self) -> &mut The<WindowData> {
        &mut self.data
    }
}
