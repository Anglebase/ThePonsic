mod types;
use std::ptr::null_mut;

use super::context::Context;
pub use types::*;
use winapi::{
    shared::windef::{HDC, HWND},
    um::{
        wingdi::*,
        winuser::*,
    },
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

impl Context2D {
    pub fn line(&self, p1: Point, p2: Point) {
        unsafe {
            MoveToEx(self.hdc, p1.x, p1.y, null_mut());
            LineTo(self.hdc, p2.x, p2.y);
        }
    }

    pub fn polyline(&self, points: &[Point]) {
        unsafe {
            Polyline(self.hdc, points.as_ptr() as _, points.len() as _);
        }
    }

    pub fn arc(&self, border: Rect, p1: Point, p2: Point) {
        unsafe {
            Arc(
                self.hdc,
                border.left,
                border.top,
                border.right,
                border.bottom,
                p1.x,
                p1.y,
                p2.x,
                p2.y,
            );
        }
    }
}
