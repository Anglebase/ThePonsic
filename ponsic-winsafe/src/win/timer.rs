use crate::check_error;
use winapi::{shared::windef::HWND, um::winuser::KillTimer};

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Timer {
    hwnd: HWND,
    uid: usize,
}

impl Timer {
    pub(crate) const fn new(hwnd: HWND, uid: usize) -> Self {
        Self { hwnd, uid }
    }

    pub fn kill(self) -> super::Result<()> {
        if unsafe { KillTimer(self.hwnd, self.uid) == 0 } {
            check_error()?;
        }
        Ok(())
    }
}
