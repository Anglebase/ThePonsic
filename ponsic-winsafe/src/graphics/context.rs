use winapi::shared::windef::HWND;

use super::context_2d::Context2D;

/// 窗口上下文
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

impl Into<Context2D<'_>> for Context {
    fn into(self) -> Context2D<'static> {
        Context2D::new(self)
    }
}
