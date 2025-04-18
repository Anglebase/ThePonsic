use ponsic_winsafe::{App, Event, Events, Recti, Return, SystemError, WindowEvent};

use super::{Proc, Window};

pub struct MainWindow {}

impl Proc for MainWindow {
    fn handle(&mut self, Events { event, .. }: Events) -> Return {
        match event {
            Event::Window(WindowEvent::Destroy) => {
                App::should_exit(0);
                Return::Finish
            }
            _ => Return::Default,
        }
    }
}

impl MainWindow {
    pub fn new(rect: Recti, title: &str) -> Result<Window, SystemError> {
        Window::create(rect, title, None, Self {})
    }
}
