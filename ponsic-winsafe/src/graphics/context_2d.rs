use super::context::Context;
use winapi::{
    shared::windef::{HDC, HWND},
    um::winuser::{BeginPaint, COLOR_WINDOW, EndPaint, FillRect, PAINTSTRUCT},
};

pub struct Context2D {
    hwnd: HWND,
    hdc: HDC,
    ps: PAINTSTRUCT,
}

impl Context2D {
    pub fn new(context: Context) -> Self {
        let mut ps = unsafe { std::mem::zeroed::<PAINTSTRUCT>() };
        let hwnd = unsafe { context.hwnd() };
        let hdc = unsafe { BeginPaint(hwnd, &mut ps) };
        Self { hwnd, hdc, ps }
    }
}

impl Drop for Context2D {
    fn drop(&mut self) {
        unsafe {
            EndPaint(self.hwnd, &self.ps);
        }
    }
}

impl Context2D {
    pub fn clear(&self) {
        unsafe {
            FillRect(self.hdc, &self.ps.rcPaint, (COLOR_WINDOW + 1) as _);
        }
    }
}
