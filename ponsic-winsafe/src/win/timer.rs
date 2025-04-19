use crate::check_error;
use winapi::{shared::windef::HWND, um::winuser::KillTimer};

#[derive(Debug)]
pub struct Timer {
    hwnd: HWND,
    uid: usize,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct TimerId {
    uid: usize,
}

impl TimerId {
    pub(crate) const fn new(uid: usize) -> Self {
        Self { uid }
    }
}

impl Timer {
    pub(crate) const fn new(hwnd: HWND, uid: usize) -> Self {
        Self { hwnd, uid }
    }

    pub fn id(&self) -> TimerId {
        TimerId::new(self.uid)
    }

    pub fn kill(self) -> super::Result<()> {
        if unsafe { KillTimer(self.hwnd, self.uid) == 0 } {
            check_error()?;
        }
        Ok(())
    }
}
