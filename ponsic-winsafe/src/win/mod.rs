use error::SystemError;

pub mod app;
pub mod class;
pub mod error;
pub mod gen_by_py;
pub mod window;
/// 模态对话框
pub mod dialog;

pub type Result<T> = std::result::Result<T, SystemError>;

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread::{sleep, spawn};
    use std::time::Duration;

    use super::*;
    use crate::events::Event;
    use crate::safe_proc::Events;
    use crate::win::app::App;
    use crate::win::class::{Cursor, PreDefineClass};
    use crate::win::window::{WindowManager, WindowStyle};
    use crate::{Return, WindowEvent, wndproc};
    use ponsic_types::{Point, Recti as Rect, Size};

    fn proc(Events { event, .. }: Events) -> Return {
        println!("{:?}", event);
        if let Event::Window(WindowEvent::Destroy) = event {
            App::should_exit(0);
            Return::Finish
        } else {
            Return::Default
        }
    }

    #[test]
    fn window_work_test() -> Result<()> {
        let class = class::Registrar::new("TestWindow1")
            .set_cursor(Cursor::Arrow)
            .set_process(wndproc!(();proc))
            .build()?;

        let window = class
            .make_window(Rect::from((Point::new(100, 100), Size::new(800, 600))))
            .set_style(&[window::WindowStyle::OverlappedWindow])
            .build()?;

        window.show();

        let exit_ = Arc::new(Mutex::new(false));
        let exit = exit_.clone();
        let join = spawn(move || {
            sleep(Duration::from_secs(1));
            *exit.lock().unwrap() = true;
        });
        while App::handle_event(false).unwrap_or(true) {
            if *exit_.lock().unwrap() {
                App::should_exit(0);
            }
        }
        join.join().unwrap();

        Ok(())
    }

    #[test]
    fn demo_work_test() -> Result<()> {
        let class = class::Registrar::new("TestWindow2")
            .set_cursor(Cursor::Arrow)
            .set_process(wndproc!(();proc))
            .build()?;

        let window = class
            .make_window(Rect::from((Point::new(100, 100), Size::new(800, 600))))
            .set_style(&[window::WindowStyle::OverlappedWindow])
            .build()?;

        let class = PreDefineClass::button();
        let btn = class
            .make_window(Rect::from((Point::new(100, 100), Size::new(100, 50))))
            .set_parent(window.id())
            .set_style(&[WindowStyle::Child])
            .set_title("Button")
            .build()?;

        window.show();
        btn.show();

        let exit_ = Arc::new(Mutex::new(false));
        let exit = exit_.clone();
        let join = spawn(move || {
            sleep(Duration::from_secs(1));
            *exit.lock().unwrap() = true;
        });
        while App::handle_event(false).unwrap_or(true) {
            if *exit_.lock().unwrap() {
                App::should_exit(0);
            }
        }
        join.join().unwrap();

        Ok(())
    }
}
