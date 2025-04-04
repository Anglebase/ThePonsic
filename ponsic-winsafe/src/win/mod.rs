use error::SystemError;

pub mod app;
pub mod class;
pub mod window;
pub mod error;

pub type Result<T> = std::result::Result<T, SystemError>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::Event;
    use crate::graphics::context_2d::Rect;
    use crate::safe_proc::Events;
    use crate::win::app::App;
    use crate::win::class::{Cursor, PreDefineClass};
    use crate::win::window::{WindowManager, WindowStyle};
    use crate::wndproc;

    fn proc(Events { event, .. }: Events) -> Option<isize> {
        println!("{:?}", event);
        if let Event::Destroy = event {
            App::should_exit(0);
            Some(0)
        } else {
            None
        }
    }

    #[test]
    fn window_work_test() -> Result<()> {
        let class = class::Registrar::new("TestWindow1")
            .set_cursor(Cursor::Arrow)
            .set_process(wndproc!(proc))
            .build()?;

        let window = class
            .window_builder(Rect::from_ps(100, 100, 800, 600))
            .set_style(&[window::WindowStyle::OverlappedWindow])
            .build()?;

        window.show();
        while App::handle_event(true).unwrap_or(true) {}
        Ok(())
    }

    #[test]
    fn demo_work_test() -> Result<()> {
        let class = class::Registrar::new("TestWindow2")
            .set_cursor(Cursor::Arrow)
            .set_process(wndproc!(proc))
            .build()?;

        let window = class
            .window_builder(Rect::from_ps(100, 100, 800, 600))
            .set_style(&[window::WindowStyle::OverlappedWindow])
            .build()?;

        let class = PreDefineClass::button();
        let btn = class
            .window_builder(Rect::from_ps(100, 100, 100, 50))
            .set_parent(window.id())
            .set_style(&[WindowStyle::Child])
            .set_title("Button")
            .build()?;

        window.show();
        btn.show();
        while App::handle_event(true).unwrap_or(true) {}
        Ok(())
    }
}
