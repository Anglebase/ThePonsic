use winapi::shared::windef::HWND;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Context {
    hwnd: HWND,
}

impl Context {
    pub unsafe fn from_raw(hwnd: HWND) -> Self {
        Self { hwnd }
    }

    pub unsafe fn hwnd(&self) -> HWND {
        self.hwnd
    }
}

