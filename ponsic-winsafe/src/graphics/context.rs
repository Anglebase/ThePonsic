use winapi::shared::windef::HWND;

use super::context_2d::Context2D;

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

impl Into<Context2D> for Context {
    fn into(self) -> Context2D {
        Context2D::new(self)
    }
}
